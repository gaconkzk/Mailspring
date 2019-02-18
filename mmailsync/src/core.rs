use std::io::{ stdin, stdout, Read, Write };
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use ctrlc::set_handler;
use std::sync::atomic::Ordering::SeqCst;
use serde_json::Value;

pub struct MMailSync {
    current_step: i8,
    account: Option<String>,
    identity: Option<String>,
}

use crate::common::*;
use serde_json::Error;

fn json_arr_2(json_arr: &Value) -> Vec<String> {
    json_arr
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|x| x.as_str())
        .map( |x| x.unwrap_or(""))
        .map(String::from)
        .collect()
}

impl MMailSync {
    pub fn new() -> MMailSync {
        MMailSync{
            current_step: 0,
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
        while running.load(SeqCst) {
            self.pre_process();
            let mut input = String::new();
            stdin().read_line(&mut input);
            self.process(input);
        }
    }
    fn pre_process(&self) {
        match self.current_step {
            0 => {
                println!("Waiting for Account JSON:");
                stdout().flush();
            },
            1 => {
                println!("Waiting for Identity JSON:");
                stdout().flush();
            },
            _ => {},
        }
    }
    fn process_account(&mut self, input: String) {
        let acc: Account = serde_json::from_str(input.as_str()).expect("Error parsing Account JSON");
        self.current_step = 1;
        stdout().flush();
    }
    fn process_identity(&mut self, input: String) {
        let id:Identity = serde_json::from_str(input.as_str()).expect("Error parsing Identity JSON");
        self.current_step = 2;
        stdout().flush();
    }
    fn process(&mut self, input: String) {
        // processing base on current step
        match self.current_step {
            0 => self.process_account(input),
            1 => self.process_identity(input),
            _ => println!("Not support current step: {}", self.current_step),
        };
    }
}
