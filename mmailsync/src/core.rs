use std::io::{ stdin, stdout, Read, Write };
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use ctrlc::set_handler;
use std::sync::atomic::Ordering::SeqCst;
use serde_json::Value;

pub struct MMailSync {
    account: Option<String>,
    identity: Option<String>,
}

use crate::models::Account;

impl MMailSync {
    pub fn new() -> MMailSync {
        MMailSync{
            account: None,
            identity: None,
        }
    }
    pub fn start(&mut self, account: & str) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        set_handler(move || {
            r.store(false, SeqCst);
        }).expect("Error setting Ctrl-C handler");
        //
        println!("Waiting for Account JSON:");
        stdout().flush();
        while running.load(SeqCst) {
            let mut input = String::new();
            stdin().read_line(&mut input);
            self.process(input);
        }
    }
    fn process(&self, input: String) {
        let json: Value = serde_json::from_str(input.as_str()).expect("Error parsing account JSON");
        match &json["__cls"] {
            Value::String(className) => {
                // store the acc
                let acc = Account {
                    id : String::from("test"),
                };
                println!("do something with {}", className);
                stdout().flush();
            },
            _=> {},
        }
//        // process input
//        match &self.account {
//            None => {
//                print!("Waiting for Account JSON:");
//                stdout().flush();
//
//                print!("{:?}", json);
//            },
//            Some(acc) => {
//                match &self.identity {
//                    None => {
//                        print!("Waiting for Identity JSON:");
//                        stdout().flush();
//                    },
//                    Some(identity) => {
//                        print!(",");
//                        stdout().flush();
//                    }
//                }
//            }
//        }
    }
}
