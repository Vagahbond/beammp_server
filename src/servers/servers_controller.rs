use rocket::http::Status;
use rocket::response::status;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use std::process::Child;
use std::sync::Mutex;

use crate::RunnerConfig;

use super::runner::Runner;
// use super::servers_service::{self, Log};

#[post("/start")]
pub fn start(state: &State<RunnerConfig>) -> String {
    *state
        .runner
        .lock()
        .as_deref_mut()
        .expect("Could not lock runner mutex") = Some(Runner::new());
    String::from("Server started successfully !")
}

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
// #[get("/logs")]
// async fn logs(queue: &State<Sender<Log>>, mut end: Shutdown) -> EventStream![] {
//     let mut rx = queue.subscribe();
//     EventStream! {
//         loop {
//             let msg = select! {
//                 msg = rx.recv() => match msg {
//                     Ok(msg) => msg,
//                     Err(RecvError::Closed) => break,
//                     Err(RecvError::Lagged(_)) => continue,
//                 },
//                 _ = &mut end => break,
//             };

//             yield Event::json(&msg);
//         }
//     }
// }

#[post("/stop")]
pub fn stop(state: &State<RunnerConfig>) -> Result<String, String> {
    state.runner.lock().as_deref_mut().unwrap().as_mut().expect("Could not access runner.").drop();
    drop(state.runner.lock().as_deref());

    *state.runner.lock().unwrap() = None;

    Ok(String::from("Server successfully stopped!"))
}
