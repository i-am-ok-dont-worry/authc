use iron::prelude::*;
use iron::{AfterMiddleware};
use iron::headers::{ContentType};
use iron::{status, Handler};
use std::net::{IpAddr};
use crate::policy::AuthCPolicy;
use log::{warn};

pub struct AuthC {
    pub policy: AuthCPolicy
}

impl AuthC {
    pub fn new(policy: &AuthCPolicy) -> AuthC {
        AuthC {
            policy: policy.clone()
        }
    }

    /// Checks if request comes from authorized origin
    pub fn check_origin_host(&self, ip: IpAddr) -> bool {
        let addr = ip.to_string();
        let is_origin_valid = self.policy.hosts.contains(&addr);

        // Allow localhost
        if addr == "127.0.0.1" {
            return true;
        }

        is_origin_valid
    }
}

impl Handler for AuthC {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let host = req.remote_addr;
        let ip = host.ip();
        let is_valid = self.check_origin_host(ip);

        Ok(Response::with((status::Ok, true)))
    }
}
