pub const EXEC: &str = "../test/BeamMP-Server-archlinux";
const CONF_ARG: &str = "--config=/src/app/ServerConfig.toml";
const DIR_ARG: &str = "--working-directory=/server_files";

// This creates some singleton

// use execute::Execute;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::State;
use std::io::{BufRead, BufReader, Read};
use std::ops::Deref;
use std::process::{Child, Command, Stdio};
use std::thread;

use super::servers_controller::ServersConfig;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Log {
    pub message: String,
}

pub fn start_server(state: &State<ServersConfig>, queue: &State<Sender<Log>>) -> String {
    let process = Command::new(EXEC)
        // .arg(CONF_ARG)
        // .arg(DIR_ARG)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn process!");

    *state.instance.lock().unwrap() = Some(process);

    // match process.stdout.unwrap().read_to_string(&mut s) {
    //     Err(why) => panic!("Could not read server stdout {}", why),
    //     Ok(_) => print!("Server replied with : \n {}", s),
    // }

    let stdout_thread = thread::spawn(move || {
        let stdout_lines = BufReader::new(
            state
                .instance
                .lock()
                .as_deref()
                .unwrap()

                .unwrap()
                .stdout.as_ref().unwrap()

        )
        .lines();

        for line in stdout_lines {
            // let line = line.unwrap();
            println!("new log: {}", line.unwrap());
        }
    });

    String::from("Started server!")
}

pub fn get_logs(process: Option<&mut Child>) -> String {
    let mut res = String::new();
    process
        .as_ref()
        .expect("Could not get running process!")
        .stdout
        .as_ref()
        .expect("Could not read stdout")
        .by_ref()
        .read_to_string(&mut res);
    res
}

pub fn stop_server(process: Option<&mut Child>) -> String {
    match process {
        Some(child) => {
            child.kill().expect("Could not kill process !");
            match child.wait() {
                Ok(output) => output.code().unwrap().to_string(),
                Err(_) => String::from("Could not stop program !"),
            }
        }
        None => String::from("Server is not running"),
    }
}
