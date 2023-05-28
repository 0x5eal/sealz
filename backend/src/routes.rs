use std::collections::HashMap;
use tide::{prelude::*, Endpoint};

fn get_routes_default() -> HashMap<&'static str, impl Endpoint<()>> {
    let mut routes = HashMap::new();

    routes.insert("GET::/api/health", |_| async move {
        Ok(json!({
            "status": 200,
            "healthy": true,
        }))
    });

    return routes;
}

pub fn get_routes() -> HashMap<&'static str, impl Endpoint<()>> {
    let routes = get_routes_default();

    // Register additional routes here

    return routes
}
