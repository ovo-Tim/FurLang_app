use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::process::Stdio;
use anyhow::anyhow;
use tokio::{io::{AsyncBufReadExt, AsyncRead, BufReader}, process::{Child, Command}};
use serde::Serialize;
use std::sync::{Arc, mpsc};
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct CommandState {
    stdout: String,
    exit_code: i32,
}

struct Cmd {
    exit_code: Arc<AtomicI32>,
    output_tx: mpsc::Sender<String>,
    child: Arc<Mutex<Child>>,
    to_kill: Arc<AtomicBool>,
    dead: Arc<AtomicBool>,
}

impl Cmd {
    fn new(output_tx: mpsc::Sender<String>, exe_path: PathBuf, arg: Option<String>, exit_code: Arc<AtomicI32>, kill: Arc<AtomicBool>, dead: Arc<AtomicBool>) -> anyhow::Result<Self>{
        let (_, child) = Self::exec_cmd(exe_path, arg)?;

        Ok(Self {
            exit_code,
            output_tx,
            child: Arc::new(Mutex::new(child)),
            to_kill: kill,
            dead,
        })
    }
    async fn wait_for_ctrlc(to_kill: Arc<AtomicBool>){
        if let Ok(_) = tokio::signal::ctrl_c().await{
            to_kill.store(true, Ordering::Relaxed);
        }
    }

    async fn wait_for_kill(to_kill: Arc<AtomicBool>, dead: Arc<AtomicBool>, child: Arc<Mutex<Child>>){
        tokio::spawn(Cmd::wait_for_ctrlc(to_kill.clone()));
        loop{
            if dead.load(Ordering::Relaxed){
                return;
            }
            if to_kill.load(Ordering::Relaxed){
                let pid = child.lock().await.id().unwrap() as i32;
                let mut done = false;
                println!("Killing by send SIGKILL to {}", pid);
                #[cfg(windows)]
                {
                    use windows::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_C_EVENT};
                    unsafe {
                        if let Ok(_) = GenerateConsoleCtrlEvent(CTRL_C_EVENT, pid as u32){
                            done = true;
                        }
                    }
                }

                #[cfg(unix)]
                {
                    use nix::sys::signal::{kill, Signal};
                    use nix::unistd::Pid;
                    if let Ok(_) = kill(Pid::from_raw(pid), Signal::SIGKILL){
                        done = true;
                    }
                }

                if !done{
                    println!("Force killing server process");
                    child.lock().await.kill().await.unwrap();
                }
                println!("Server killed");
                dead.store(true, Ordering::Relaxed);
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    async fn read_and_send<T:AsyncRead + Unpin>(&mut self, reader: &mut BufReader<T>) -> anyhow::Result<()> {
        let mut empty_count = 0;
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).await?;
            if buf.is_empty(){
                empty_count += 1;
                if empty_count > 5{
                    return Err(anyhow!("Empty output"));
                }
                continue;
            }
            println!("Server: {}", buf);
            self.output_tx.send(buf)?;
            empty_count = 0;
        }
    }

    async fn state_watch(&mut self){
        let mut child = self.child.lock().await;
        let mut stdout_reader = tokio::io::BufReader::new(child.stdout.take().unwrap());
        let mut stderr_reader = tokio::io::BufReader::new(child.stderr.take().unwrap());
        let mut err_count = 0;
        tokio::spawn(Cmd::wait_for_kill(self.to_kill.clone(), self.dead.clone(), self.child.clone()));
        drop(child);
        while err_count < 5{
            if let Err(e) = self.read_and_send(&mut stdout_reader).await{
                println!("Failed to read server output: {}", e);
                if self.to_kill.load(Ordering::Relaxed){
                    return;
                }
                if self.check_status().await{
                    break;
                }
            }
            err_count += 1;
        }
        println!("State watch exit");
        let _ = self.read_and_send(&mut stderr_reader).await;
        self.check_status().await;
    }

    async fn check_status(&mut self) -> bool{
        // Check if the child process has exited and write the exit code to the exit_code variable.
        let mut child = self.child.lock().await;
        println!("check_status");
        if let Ok(Some(exit_status)) = child.try_wait(){
            if let Some(code) = exit_status.code(){
                println!("Exit code wrote: {}", code);
                self.exit_code.store(code, Ordering::Relaxed);
                self.dead.store(true, Ordering::Relaxed);
                return true;
            }
        }
        return false;
    }

    fn exec_cmd(exe_path: PathBuf, arg: Option<String>) -> anyhow::Result<(Command, Child)> {
        let exe_dir = exe_path.parent();
        if exe_dir.is_none(){
            return Err(anyhow!("Failed to get server path"));
        }
        let mut cmd = Command::new(&exe_path);
        cmd.current_dir(exe_dir.unwrap()).stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::piped());
        cmd.kill_on_drop(true);
        if let Some(_arg) = arg{
            cmd.arg(_arg);
        }
        match cmd.spawn() {
            Ok(child) => {
                println!("Created {} at {}", exe_path.display(), exe_dir.unwrap().display());
                Ok((cmd, child))
            },
            Err(e) => {
                println!("Failed to start server: {}", e);
                Err(anyhow!("Failed to start server"))
            }
        }
    }
}

pub struct CommandRunner{
    output_rx: Option<mpsc::Receiver<String>>,
    exit_code: Arc<AtomicI32>,
    kill: Arc<AtomicBool>,
    dead: Arc<AtomicBool>,
}

impl CommandRunner{
    pub fn new() -> Self{
        Self{
            output_rx: None,
            exit_code: Arc::new(AtomicI32::new(-1)),
            kill: Arc::new(AtomicBool::new(false)),
            dead: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_server(&mut self, exe_path: PathBuf, arg: Option<String>) -> Result<(), String>{
        let (tx, rx) = mpsc::channel();
        self.output_rx = Some(rx);
        let mut cmd = match Cmd::new(tx, exe_path, arg, self.exit_code.clone(),self.kill.clone(), self.dead.clone()) {
            Ok(cmd) => cmd,
            Err(e) => return Err(e.to_string())
        };
        tokio::spawn(async move {
            cmd.state_watch().await;
        });
        Ok(())
    }

    pub fn get_state(&mut self) -> Result<CommandState, String>{
        let mut out_buf = String::new();
        let exit_code = self.exit_code.load(Ordering::Relaxed);

        loop{
            match self.output_rx.as_mut().unwrap().try_recv(){
                Ok(msg) => out_buf.push_str(&msg),
                Err(_) => break,
            }
        }
        // println!("Server: {}", &out_buf);

        Ok(CommandState{
            stdout: out_buf,
            exit_code
        })
    }

    pub fn kill(&mut self){
        self.kill.store(true, Ordering::Relaxed);
        while !self.dead.load(Ordering::Relaxed){
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
