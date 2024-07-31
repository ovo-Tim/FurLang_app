use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::process::Stdio;
use anyhow::anyhow;
use tokio::{io::{AsyncBufReadExt, AsyncRead, BufReader}, process::{Child, Command}};
use serde::Serialize;
use std::sync::{Arc, mpsc};

#[derive(Serialize)]
pub struct CommandState {
    stdout: String,
    exit_code: Option<i32>,
}


pub struct CommandRunner{
    exited: Arc<AtomicBool>,
    exit_code: Arc<AtomicI32>,
    rx: mpsc::Receiver<String>,
    tx: mpsc::Sender<String>,
}

impl CommandRunner{
    pub fn new() -> Self{
        let (stdout_tx, stdout_rx) = mpsc::channel();
        Self{
            exited: Arc::new(AtomicBool::new(false)),
            exit_code: Arc::new(AtomicI32::new(-1)),
            rx: stdout_rx,
            tx: stdout_tx,
        }
    }

    fn create(exe_path: PathBuf, arg: Option<String>) -> anyhow::Result<(Command, Child)>{
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
        // println!("read_and_send");
        let mut buf = String::new();
        reader.read_line(&mut buf).await?;
        if buf.is_empty(){
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            return Err(anyhow!("Stdout has closed and there is nothing to read"));
        }
        println!("Server: {}", buf);
        tx.send(buf).unwrap();
        Ok(())
    }

    async fn check_status(child: &mut Child, exited: &Arc<AtomicBool>, exit_code: &Arc<AtomicI32>, tx: &mpsc::Sender<String>){
        println!("check_status");
        if let Ok(Some(exit_status)) = child.try_wait(){
            exited.store(true,  Ordering::Relaxed);
            if let Some(code) = exit_status.code(){
                exit_code.store(code, Ordering::Relaxed);
            }
        }
        let mut stderr = tokio::io::BufReader::new(child.stderr.take().unwrap());
        let _ = CommandRunner::read_and_send(&mut stderr, tx).await;
    }

    async fn state_watch(mut child: Child, tx: mpsc::Sender<String>, exited: Arc<AtomicBool>, exit_code: Arc<AtomicI32>){
        let mut stdout = tokio::io::BufReader::new(child.stdout.take().unwrap());
        loop{
            match CommandRunner::read_and_send(&mut stdout, &tx).await{
                Ok(_) => {
                    continue;
                },
                Err(e) => {
                    println!("Failed to read server output: {}", e);
                    CommandRunner::check_status(&mut child, &exited, &exit_code, &tx).await;
                }
            }
            break;
        }
    }

    pub fn start_server(&mut self, exe_path: PathBuf, arg: Option<String>) -> Result<(), String>{
        if let Ok((_, child)) = Self::create(exe_path.clone(), arg){
            tokio::spawn(Self::state_watch(child, self.tx.clone(), self.exited.clone(), self.exit_code.clone()));
            return Ok(());
        }
        Err("Failed to start server".into())
    }

    pub fn get_state(&mut self) -> Result<CommandState, String>{
        let mut out_buf = String::new();
        let mut exit_code = None;
        if self.exited.load(Ordering::Relaxed){
            exit_code = Some(self.exit_code.load(Ordering::Relaxed));
        }
        loop{
            if let Ok(stdout) = self.rx.try_recv(){
                if !stdout.is_empty(){
                    out_buf.push_str(&stdout);
                    continue; // Only recviving can make the loop alive, empty value or Err will break the loop.
                }
            }
            break;
        }
        // println!("Server: {}", &out_buf);

        Ok(CommandState{
            stdout: out_buf,
            exit_code
        })
    }
}
