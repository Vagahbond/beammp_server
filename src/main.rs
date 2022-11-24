pub mod mods;
pub mod servers;

use std::sync::{Mutex, PoisonError};

use servers::runner::Runner;

#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> String {
    String::from("Hello world, your Beam MP server is working !")
}

pub struct RunnerConfig {
    pub runner: Mutex<Option<Runner>>,
}

impl RunnerConfig {
    pub fn new() {
        RunnerConfig {
            runner: Mutex::new(None),
        };
    }

    pub fn start(&mut self) -> String {
        *self
            .runner
            .lock()
            .as_deref_mut()
            .expect("Could not lock runner mutex") = Some(Runner::new());
        String::from("Server started successfully !")
    }

    // pub fn stop(&mut self) -> Result<String, String> {
    //     let current_runner = match self.runner.lock().as_deref() {
    //         Ok(r) => r.unwrap(),
    //         Err(e) => return Err(String::from("Mutex error occured accessing runner.")),
    //     };

    //     drop(current_runner);

    //     *self.runner.lock().unwrap() = None;

    //     Ok(String::from("Server successfully stopped!"))
    // }
}

#[launch]
fn rocket() -> _ {
    let servers_config = RunnerConfig {
        runner: Mutex::new(Some(Runner::new())),
    };

    rocket::build()
        .mount("/", routes![home])
        .mount(
            "/servers",
            routes![
                servers::servers_controller::start,
                servers::servers_controller::stop
            ],
        )
        .mount("/mods", routes![])
        .manage(servers_config)
}
