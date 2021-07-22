extern crate router;
extern crate logger;

use log::{info, warn};
use logger::Logger;
use crate::policy::PolicyReader;
use router::Router;
use iron::prelude::*;

mod policy;
mod middleware;
mod authc;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    info!("Hello from logger");

    let p = policy::PolicyReader::new("policy.json");
    match p.validate() {
        Ok(_) => println!("Validation ok"),
        Err(e) => println!("Validation failed {:?}", e)
    }

    // Create server router
    let mut router = Router::new();

    // Create AuthC middleware
    let authc_middleware = authc::AuthC::new();

    // Declare routes
    router.get("/", authc_middleware, "index");

    // Create middleware chain
    let mut chain = Chain::new(router);

    // Create json content-type header middleware
    let json_content_type_middleware = middleware::JsonAfterMiddleware;

    // Create Iron logger
    let (logger_before, logger_after) = Logger::new(None);

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);
    chain.link_after(json_content_type_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
