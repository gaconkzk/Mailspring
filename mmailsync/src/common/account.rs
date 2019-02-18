use serde::{ Serialize, Deserialize };
use crate::common::Settings;
use crate::common::Address;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub aliases: Vec<String>,
    pub __cls: String,
    pub autoaddress: Address,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub label: String,
    pub metadata: Vec<String>,
    pub name: String,
    pub provider: String,
    pub settings: Settings,
    #[serde(rename = "syncError")]
    pub sync_error: Option<String>,
    #[serde(rename = "syncState")]
    pub sync_state: Option<String>,
}