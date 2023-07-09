use std::collections::HashMap;

use tiny_http::{Method, Request, Response, Server};
use tokio::task::JoinHandle;

use crate::config;
use crate::spotify;

pub fn listen() -> JoinHandle<()> {
    let server = Server::http("0.0.0.0:3000").expect("Failed to create server");
    tokio::spawn(async move {
        for request in server.incoming_requests() {
            let method = request.method();
            match method {
                | Method::Get => {
                    let url = request.url();
                    let route = parse_url(url, 0);
                    match route {
                        | Some(_) => {
                            let query_params = parse_query_params(url);
                            println!("{query_params:#?}");
                            let token = spotify::get_oauth_token().await.unwrap();
                            let config =
                                config::update(token.oauth_token, token.oauth_refresh_token);
                            //                     let response = Response::from_string("pong");
                            //                     let _ = request.respond(response);
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

fn parse_query_params(url: &str) -> Option<HashMap<&str, &str>> {
    let query_param_string = parse_url(url, 1);
    match query_param_string {
        | Some(qp) => {
            let mut query_params: HashMap<&str, &str> = HashMap::new();
            let parts = qp.split("&");
            for part in parts {
                let kv = part.split("=").into_iter().collect::<Vec<&str>>();
                if kv.get(0).is_some() && kv.get(1).is_some() {
                    query_params.insert(kv.get(0).unwrap(), kv.get(1).unwrap());
                }
            }
            Some(query_params)
        },
        | None => None,
    }
}
