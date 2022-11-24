
pub mod servers;
pub mod mods;

use std::sync::Mutex;

#[macro_use] extern crate rocket;

#[get("/")]
fn home() -> String {
    String::from("Hello world, your Beam MP server is working !")
}




#[launch]
fn rocket() -> _ {
    let servers_config = servers::servers_controller::ServersConfig {
        instance: Mutex::new(None)
    };

    rocket::build()
        .mount("/", routes![home])
        .mount("/servers", routes![servers::servers_controller::start, servers::servers_controller::stop])
        .mount("/mods", routes![])
        .manage(servers_config)

}