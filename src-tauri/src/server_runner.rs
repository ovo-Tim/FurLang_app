use std::path::PathBuf;

use tokio::{io::AsyncReadExt, process::{self, Command}};
use serde::Serialize;

#[derive(Serialize)]
pub struct CommandState {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
}

#[derive(Default)]
pub struct CommandRunner{
    cmd: Option<process::Child>,
}

impl CommandRunner{
    pub fn start_server(&mut self, exe_path: PathBuf)->Result<(), String>{
        let exe_dir = exe_path.parent();
        if exe_dir.is_none(){
            return Err("Failed to get server path".into());
        }
        let mut _tmp = Command::new(&exe_path);
        let mut _cmd = _tmp.current_dir(exe_dir.unwrap());
        _cmd.kill_on_drop(true);
        if let Ok(cmd) = _cmd.spawn(){
            self.cmd = Some(cmd);
            Ok(())
        }else{
            Err("Failed to start server".into())
        }
    }

    pub fn get_state(&mut self) -> Result<CommandState, String>{
        if self.cmd.is_none(){
            return Err("Failed to get state. You may not start the server yet.".into());
        }

        let cmd = self.cmd.as_mut().unwrap();
        let mut out_buf = String::new();
        let mut err_buf = String::new();
        let mut exit_code = None;
        if let Some(output) = &mut cmd.stdout{
            let _ = output.read_to_string(&mut out_buf);
        }
        if let Some(output) = &mut cmd.stderr{
            let _ = output.read_to_string(&mut err_buf);
        }
        if let Ok(Some(code)) = cmd.try_wait(){
            exit_code = code.code();
        }
        Ok(CommandState{
            stdout: out_buf,
            stderr: err_buf,
            exit_code
        })
    }
}