use std::io::{Read, stdin, stdout, Write};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

use ctrlc::set_handler;
use serde_json::Error;
use serde_json::Value;

use crate::common::*;

#[derive(Clone)]
pub struct MMailSync {
    current_step: i8,
    account: Option<String>,
    identity: Option<String>,
}

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
        //
        let s_clone = self.clone();
        let r = running.clone();
        let syncer = thread::spawn(move || {
            // Handling stdin input
            while r.load(SeqCst) {
                thread::sleep(Duration::from_secs(5));
            }
        });
        let mut s_clone = self.clone();
        let r = running.clone();
        let processor = thread::spawn(move || {
            // Handling stdin input
            while r.load(SeqCst) {
                s_clone.pre_process();
                let mut input = String::new();
                stdin().read_line(&mut input);
                s_clone.process(input);
            }
        });
        syncer.join().unwrap();
        processor.join().unwrap();
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
