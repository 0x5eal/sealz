#![feature(let_chains)]

use colored::Colorize;
use std::{collections::HashMap, net::SocketAddr};
use tide::{Endpoint, Result};

pub enum ReqType {
    GET,
    POST,
}

pub struct ServerOptions<'a, T> {
    pub to_expose: bool,
    pub exposed_port: Option<u16>,
    pub bulk_routes: Option<HashMap<&'a str, T>>,
    pub scope: &'a str,
}

/// This struct may be public, but is for internal use only.
/// To instantiate a server, use `setup_server` instead. It may
/// be acceptable to use `Server.add_route` though.
pub struct Server {
    instance: tide::Server<()>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            instance: tide::new(),
        }
    }

    pub fn add_route(&mut self, method: ReqType, route: &str, handler: impl Endpoint<()>) {
        let mut route_addr = self.instance.at(route);

        match method {
            ReqType::GET => route_addr.get(handler),
            ReqType::POST => route_addr.post(handler),
        };

        println!(
            "{} :: added route {} to routes collection",
            "[framework]".bold(),
            route.blue().underline()
        );
    }

    pub async fn expose(&self, scope: &str, port: u16) -> Result<()> {
        self.instance
            .clone()
            .listen(SocketAddr::from(([127, 0, 0, 1], port)))
            .await
            .and_then(|entry| {
                println!(
                    "{} :: {} service listening at port {}",
                    "[framework]".bold(),
                    scope.green(),
                    port.to_string().yellow().bold()
                );

                Ok(entry)
            })?;

        Ok(())
    }
}

pub async fn setup_server(
    opts: ServerOptions<'static, impl Endpoint<()>>,
) -> std::result::Result<Server, tide::Error> {
    let mut server = Server::new();

    if let Some(routes_map) = opts.bulk_routes {
        println!(
            "{} :: setting up server...",
            "[framework_bootstrapper]".bold()
        );

        for (route, handler) in routes_map {
            let meta: Vec<&str> = route.split("::").collect();

            let method = match meta[0] {
                "GET" => ReqType::GET,
                "POST" => ReqType::POST,

                &_ => panic!("framework::setup_serer::bulk_routes -> invalid request method type"),
            };

            let route = meta[1];

            server.add_route(method, route, handler);
        }
    }

    if opts.to_expose && let Some(port) = opts.exposed_port {
        server.expose(opts.scope, port).await?;
    }

    Ok(server)
}
