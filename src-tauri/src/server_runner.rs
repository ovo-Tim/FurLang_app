use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::process::Stdio;
use anyhow::anyhow;
use tokio::{io::{AsyncBufReadExt, AsyncRead}, process::{Child, Command}};
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

    fn create(exe_path: PathBuf) -> anyhow::Result<(Command, Child)>{
        let exe_dir = exe_path.parent();
        if exe_dir.is_none(){
            return Err(anyhow!("Failed to get server path"));
        }
        let mut cmd = Command::new(&exe_path);
        cmd.current_dir(exe_dir.unwrap()).stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd.kill_on_drop(true);
        if let Ok(child) = cmd.spawn(){
            Ok((cmd, child))
        }else{
            Err(anyhow!("Failed to start server"))
        }
    }

    async fn readline(stdout: &mut (impl AsyncRead + Unpin + 'static)) -> anyhow::Result<String>{
        let mut buf = String::new();
        let reader = tokio::io::BufReader::new(stdout);
        let mut pinned_reader = Box::pin(reader);
        pinned_reader.read_line(&mut buf).await?;
        Ok(buf)
    }

    fn check_status(child: &mut Child, exited: &Arc<AtomicBool>, exit_code: &Arc<AtomicI32>){
        if let Ok(Some(exit_status)) = child.try_wait(){
            exited.store(true, Ordering::Relaxed);
            if let Some(code) = exit_status.code(){
                exit_code.store(code, Ordering::Relaxed);
            }
        }
    }

    async fn state_watch(mut child: Child, tx: mpsc::Sender<String>, exited: Arc<AtomicBool>, exit_code: Arc<AtomicI32>){
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();
        loop{
            if let Ok(buf) = CommandRunner::readline(&mut stderr).await{
                tx.send(buf).unwrap();
            }
            match CommandRunner::readline(&mut stdout).await{
                Ok(buf) => {
                    tx.send(buf).unwrap();
                    continue;
                },
                Err(_) => CommandRunner::check_status(&mut child, &exited, &exit_code)
            }
            break;
        }
    }

    pub fn start_server(&mut self, exe_path: PathBuf) -> Result<(), String>{
        if let Ok((_, child)) = Self::create(exe_path.clone()){
            tokio::spawn(Self::state_watch(child, self.tx.clone(), self.exited.clone(), self.exit_code.clone()));
            return Ok(());
        }
        Err("Failed to start server".into())
    }

    pub async fn get_state(&mut self) -> Result<CommandState, String>{
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

        Ok(CommandState{
            stdout: out_buf,
            exit_code
        })
    }
}