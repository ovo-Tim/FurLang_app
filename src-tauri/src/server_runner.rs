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
    exit_code: Option<i32>,
}


pub struct CommandRunner{
    exit_code: Arc<AtomicI32>,
    output_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
    kill: Arc<AtomicBool>,
}

impl CommandRunner{
    pub fn new() -> Self{
        let (stdout_tx, stdout_rx) = mpsc::channel();
        Self{
            exit_code: Arc::new(AtomicI32::new(-1)),
            output_rx: stdout_rx,
            output_tx: stdout_tx,
            kill: Arc::new(AtomicBool::new(false)),
        }
    }

    fn exec_cmd(exe_path: PathBuf, arg: Option<String>) -> anyhow::Result<(Command, Child)>{
        let exe_dir = exe_path.parent();
        if exe_dir.is_none(){
            return Err(anyhow!("Failed to get server path"));
        }
        let mut cmd = Command::new(&exe_path);
        cmd.current_dir(exe_dir.unwrap()).stdout(Stdio::piped()).stderr(Stdio::piped());
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

    async fn read_and_send<T:AsyncRead + Unpin>(reader: &mut BufReader<T>, tx: &mpsc::Sender<String>) -> anyhow::Result<()> {
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
            tx.send(buf)?;
            empty_count = 0;
        }
    }

    async fn check_status(child: Arc<Mutex<Child>>, exit_code: &Arc<AtomicI32>) -> bool{
        // Check if the child process has exited and write the exit code to the exit_code variable.
        let mut child = child.lock().await;
        println!("check_status");
        if let Ok(Some(exit_status)) = child.try_wait(){
            if let Some(code) = exit_status.code(){
                println!("Exit code wrote: {}", code);
                exit_code.store(code, Ordering::Relaxed);
                return true;
            }
        }
        return false;
    }

    async fn wait_for_ctrlc(kill: Arc<AtomicBool>){
        if let Ok(_) = tokio::signal::ctrl_c().await{
            kill.store(true, Ordering::Relaxed);
        }
    }

    async fn wait_for_kill(child: Arc<Mutex<Child>>, kill: Arc<AtomicBool>){
        tokio::spawn(CommandRunner::wait_for_ctrlc(kill.clone()));
        loop{
            if kill.load(Ordering::Relaxed){
                let _ = child.lock().await.kill().await;
                println!("Server killed");
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    async fn state_watch(mut child: Child, tx: mpsc::Sender<String>, exit_code: Arc<AtomicI32>, kill: Arc<AtomicBool>){
        let mut stdout_reader = tokio::io::BufReader::new(child.stdout.take().unwrap());
        let mut stderr_reader = tokio::io::BufReader::new(child.stderr.take().unwrap());
        let mut err_count = 0;
        let child = Arc::new(Mutex::new(child));
        tokio::spawn(CommandRunner::wait_for_kill(child.clone(), kill.clone()));
        while err_count < 5{
            if let Err(e) = CommandRunner::read_and_send(&mut stdout_reader, &tx).await{
                println!("Failed to read server output: {}", e);
                if kill.load(Ordering::Relaxed){
                    return;
                }
                if CommandRunner::check_status(child.clone(), &exit_code).await{
                    break;
                }
            }
            err_count += 1;
        }
        println!("State watch exit");
        let _ = CommandRunner::read_and_send(&mut stderr_reader, &tx).await;
        CommandRunner::check_status(child.clone(), &exit_code).await;
    }

    pub fn start_server(&mut self, exe_path: PathBuf, arg: Option<String>) -> Result<(), String>{
        if let Ok((_, child)) = Self::exec_cmd(exe_path.clone(), arg){
            tokio::spawn(Self::state_watch(child, self.output_tx.clone(), self.exit_code.clone(), self.kill.clone()));
            return Ok(());
        }
        Err("Failed to start server".to_string())
    }

    pub fn get_state(&mut self) -> Result<CommandState, String>{
        let mut out_buf = String::new();
        let exit_code = Some(self.exit_code.load(Ordering::Relaxed));

        loop{
            match self.output_rx.try_recv(){
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
    }
}
