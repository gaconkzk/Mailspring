use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

use ctrlc::set_handler;

use crate::common::*;

#[derive(PartialEq, Clone)]
pub struct MMailSync {
    account: Option<String>,
    identity: Option<String>,
    in_server: Option<MailClient>,
}

impl MMailSync {
    pub fn new() -> MMailSync {
        MMailSync{
            account: None,
            identity: None,
            in_server: None,
        }
    }
    // we need account live long enough until start finished.
    pub fn start(&mut self, account: &str) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        set_handler(move || {
            r.store(false, SeqCst);
        }).expect("Error setting Ctrl-C handler");

        // todo check account validity
        self.account = Some(String::from(account));
        //
        if self.in_server == None {
            self.wait_account_input();
        }
        if self.identity == None {
            self.wait_identity_input();
        }

        // initialize imap connection
        match &self.in_server {
            Some(imap) => imap.syncing_server().unwrap(),
            None => panic!("Error getting IMAP server information"),
        };
        // getting list of folders

        // Handling stdin input
        while running.load(SeqCst) {
            println!("syncing");
            thread::sleep(Duration::from_secs(5));
        }
    }
    fn wait_account_input(&mut self) {
        println!("Waiting for Account JSON:");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error waiting account json input");
        self.process_account(input);
    }
    fn wait_identity_input(&mut self) {
        println!("Waiting for Identity JSON:");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error waiting identity json input");
        self.process_identity(input);
    }
    fn process_account(&mut self, input: String) {
        let acc: Account = serde_json::from_str(input.as_str()).expect("Error parsing Account JSON");
        self.in_server = MailClient::new(acc.settings);
    }
    fn process_identity(&mut self, input: String) {
        let id:Identity = serde_json::from_str(input.as_str()).expect("Error parsing Identity JSON");
        // todo identity for commercial product - not consider here.
        self.identity = Some(id.id);
    }
}
