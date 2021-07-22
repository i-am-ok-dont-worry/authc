use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::process;
use std::collections::HashMap;
use std::collections::BTreeMap;
use serde_json::{Value, Result, from_value};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthCPolicy {
    hosts: Vec<String>
}

pub struct PolicyReader {
    pub policy: Value,
    pub hosts: Vec<String>
}

impl PolicyReader {
    pub fn new(filepath: &str) -> PolicyReader {
        let mut file = File::open(filepath)
            .unwrap_or_else(|_| {
                println!("Error while reading a policy file. Create a policy.json file");
                process::exit(1);
            });

        let policy: Value = serde_json::from_reader(&file).unwrap();
        let policy_clone = policy.clone();
        let hosts: Vec<String> = from_value(policy_clone["hosts"].to_owned()).unwrap();

        let reader = PolicyReader {
            policy,
            hosts
        };

        reader
    }

    pub fn validate(&self) -> std::result::Result<(), &'static str> {

        /// Check 'hosts' policy property - this field should only accept array of valid strings
        let is_hosts_property_valid: std::result::Result<(), &'static str> = match self.policy.get("hosts") {
            Some(val) => {
                match val {
                    Value::Array(server) => {
                        let i = server.iter().for_each(|val| {
                            if val.to_string().len() <= 2 {
                                panic!("Invalid value for property 'hosts': {:?}. Only valid strings are acceptable", val);
                            }
                        });
                        Ok(())
                    },
                    _ => return Err("Invalid value for property 'hosts'. Must be a valid array of strings")
                }
            },
            None => return Err("Cannot read property hosts. Property 'servers' must be a valid string array")
        };


        Ok(())
    }
}
