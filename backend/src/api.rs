use std::collections::HashMap;
use tide::Endpoint;

mod routes;

pub fn get_routes_default() -> HashMap<&'static str, impl Endpoint<()>> {
    let mut routes = HashMap::new();

    routes.insert("GET::/api/random", routes::random::handler);

    return routes;
}
