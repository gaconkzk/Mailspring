use serde::{ Serialize, Deserialize };
use tokio_imap::{ ImapClient, TlsClient };

use crate::common::Settings;
use futures::future::Future;
use tokio_imap::client::builder::CommandBuilder;
use futures_state_stream::StateStream;

use std::io;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::result::Result;
use tokio_imap::proto::ResponseData;
use std::error::Error;
use std::io::stdout;
use std::io::Write;

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct MailClient {
    pub allow_insecure_ssl: bool,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub security: String,
    pub username: String,
    _server_type: MailProtocolType,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum MailProtocolType {
    IMAP,
    SMTP,
    POP
}

impl MailClient {
    pub fn new(settings: Settings) -> Option<MailClient> {
        Some(MailClient {
            allow_insecure_ssl: settings.imap_allow_insecure_ssl,
            host: settings.imap_host.clone(),
            port: settings.imap_port,
            username: settings.imap_username.clone(),
            password: settings.imap_password.clone(),
            security: settings.imap_security.clone(),
            _server_type: MailProtocolType::IMAP,
        })
    }
    pub fn syncing_server(&self) -> Result<(), ImapError> {
        // connect to the server
        let server: String = format!("{}:{}", self.host, self.port);
        let fut_connect = TlsClient::connect(server.as_str()).map_err(|cause| ImapError::Connect { cause })?;
        let fut_responses = fut_connect
            .and_then(move |(_, tls_client)| {
                tls_client
                    .call(CommandBuilder::login(self.username.as_str(), self.password.as_str()))
                    .collect()
            })
            .and_then(move |(_, tls_client)| {
                tls_client
                    .call(CommandBuilder::list("", ""))
                    .collect()
            })
            .and_then(move |(data, tls_client)| {
                process_data(data);
                tls_client.call(CommandBuilder::close()).collect()
            })
            .and_then(|_| Ok(()))
            .map_err(|e| ImapError::Select { cause: e });

        let res:Result<(), ImapError> = tokio_current_thread::block_on_all({
            eprintln!("Fetching message...");
            fut_responses
        });
        eprintln!("Finished fetching messages");
        res
    }
}

fn process_data(responses: Vec<ResponseData>) {
    responses.iter()
        .for_each(|it| {
            println!("{:?}", it.parsed());
        });
}

#[derive(Debug)]
pub enum ImapError {
    Connect { cause: io::Error },
    Login { cause: io::Error },
    Select { cause: io::Error },
    UidFetch { cause: io::Error },
}

impl Error for ImapError {
    fn description(&self) -> &'static str {
        ""
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ImapError::Connect { ref cause }
            | ImapError::Login { ref cause }
            | ImapError::Select { ref cause }
            | ImapError::UidFetch { ref cause } => Some(cause),
        }
    }
}

#[macro_use]
impl Display for ImapError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ImapError::Connect { ref cause } => pflush!("Connect failed: {}", cause),
            ImapError::Login { ref cause } => pflush!("Login failed: {}", cause),
            ImapError::Select { ref cause } => pflush!("Mailbox selection failed: {}", cause),
            ImapError::UidFetch { ref cause } => pflush!("Fetching messages failed: {}", cause),
        }
    }
}

macro_rules! pflush {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        std::io::_print(format_args_nl!($($arg)*));
        std::io::stdout().flush().unwrap();
    })
}