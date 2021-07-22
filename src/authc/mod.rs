use iron::prelude::*;
use iron::{AfterMiddleware};
use iron::headers::{ContentType};
use iron::{status, Handler};

pub struct AuthC {}

impl AuthC {
    pub fn new() -> AuthC {
        AuthC {}
    }
}

impl Handler for AuthC {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let host = req.remote_addr;
        let ip = host.ip();
        println!("Remote host: {:?}", ip);
        Ok(Response::with((status::Ok, "AuthC OK")))
    }
}
