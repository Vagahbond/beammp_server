const EXEC: &str = "/src/app/BeamMP-Server-debian-11";
const CONF_ARG: &str = "--config=/src/app/ServerConfig.toml";
const DIR_ARG: &str = "--working-directory=/server_files";

use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

pub struct Runner {
    pub handle: Child,
    pub output: Arc<Mutex<String>>,
    output_thread: JoinHandle<()>,
}

impl Runner {
    pub fn new() -> Runner {
        let mut handle = Command::new(EXEC)
            .arg(CONF_ARG)
            .arg(DIR_ARG)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Could not spawn process!");

        let mut stdout = handle.stdout.take().unwrap();
        let output = Arc::new(Mutex::new(String::new()));
        let out_clone = Arc::clone(&output);

        let output_thread = spawn(move || loop {
            let mut buf: [u8; 1] = [0];
            let mut output = out_clone.lock().unwrap();

            let read = stdout.read(&mut buf);
            match read {
                Err(err) => {
                    println!("{}] Error reading from stream : {}", line!(), err);
                    break;
                }
                Ok(bytes_read) => {
                    if bytes_read != 0 {
                        let char = String::from_utf8(buf.to_vec()).unwrap();
                        output.push_str(char.as_str());
                    } else if output.len() != 0 {
                        out_clone.lock().unwrap().clear();
                    }
                }
            }
        });

        Runner {
            handle,
            output,
            output_thread,
        }
    }

    pub fn drop(&mut self) {

        self.handle.kill().expect("Could not kill server process!");
        // self.output_thread.join();
        match self.handle.wait() {
            Ok(output) => {
                match output.code() {
                    Some(code) => println!("Proxess exited with code : {}", code),
                    None => eprintln!("Could not get process exit code.")
                }
            }
            Err(err) => {
                eprintln!("Could not close server process : {}", err.to_string());
            }
        }
    }
}
