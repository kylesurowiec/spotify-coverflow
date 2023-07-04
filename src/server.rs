use std::thread::JoinHandle;

use tiny_http::{Response, Server};

pub fn listen() -> JoinHandle<()> {
    let server = Server::http("0.0.0.0:3000").expect("Failed to create server");
    std::thread::spawn(move || {
        for request in server.incoming_requests() {
            let response = Response::from_string("pong");
            let _ = request.respond(response);
        }
    })
}
