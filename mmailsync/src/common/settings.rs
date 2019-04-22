use serde::{ Serialize, Deserialize };

pub use self::mail_client::MailClient;
pub use self::mail_client::MailProtocolType;

pub mod mail_client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub imap_allow_insecure_ssl: bool,
    pub imap_host: String,
    pub imap_password: String,
    pub imap_port: u16,
    pub imap_security: String,
    pub imap_username: String,

    pub smtp_allow_insecure_ssl: bool,
    pub smtp_host: String,
    pub smtp_password: String,
    pub smtp_port: u16,
    pub smtp_security: String,
    pub smtp_username: String,
}