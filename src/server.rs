use std::collections::HashMap;
use std::thread::JoinHandle;

use anyhow::Result;
use tiny_http::{Method, Request, Response, Server};

pub fn listen() -> JoinHandle<()> {
    let server = Server::http("0.0.0.0:3000").expect("Failed to create server");
    std::thread::spawn(move || {
        for request in server.incoming_requests() {
            match request.method() {
                | Method::Get => {
                    let url = request.url();
                    let route = parse_url(url, 0);
                    match route {
                        | Some(_) => {
                            let query_params = parse_query_params(url);
                            let response = Response::from_string("pong");
                            let _ = request.respond(response);
                        },
                        | None => respond_404(request),
                    }
                },
                | _ => {
                    respond_404(request);
                },
            }
        }
    })
}

fn respond_404(request: Request) {
    let _ = request.respond(Response::from_data([]).with_status_code(404));
}

fn parse_url(url: &str, index: usize) -> Option<&str> {
    let route = url.split("?").into_iter().collect::<Vec<&str>>();
    match route.get(index) {
        | Some(route) => Some(*route),
        | None => None,
    }
}

fn parse_query_params(url: &str) -> Result<HashMap<&str, &str>> {
    let mut query_params: HashMap<&str, &str> = HashMap::new();
    let query_param_string = parse_url(url, 1);

    match query_param_string {
        | Some(qp) => {
            let parts = qp.split("&");
            for part in parts {
                let kv = part.split("=").into_iter().collect::<Vec<&str>>();
                if kv.get(0).is_some() && kv.get(1).is_some() {
                    query_params.insert(kv.get(0).unwrap(), kv.get(1).unwrap());
                }
            }
        },
        | None => {},
    };

    Ok(query_params)
}
