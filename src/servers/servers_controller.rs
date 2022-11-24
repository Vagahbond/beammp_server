use std::process::Child;
use std::sync::Mutex;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::{State, Shutdown};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;


use super::servers_service::{self, Log};
pub struct ServersConfig {
    pub instance: Mutex<Option<Child>>,
}

#[post("/start")]
pub fn start(state: &State<ServersConfig>, queue: &State<Sender<Log>>) -> String {
    match &*state.instance.lock().unwrap() {
        Some(_) => String::from("Server is already on !"),
        None => {
            servers_service::start_server(state, queue)
        }
    }
}

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/logs")]
async fn logs(queue: &State<Sender<Log>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[post("/stop")]
pub fn stop(state: &State<ServersConfig>) -> String {
    servers_service::stop_server(state.instance.lock().as_deref_mut().unwrap().as_mut())
}
