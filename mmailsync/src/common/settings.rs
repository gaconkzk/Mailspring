use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct MailProtocol {
    pub allow_insecure_ssl: bool,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub security: String,
    pub username: String,
}

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