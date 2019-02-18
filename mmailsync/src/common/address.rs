use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "type")]
    type_: String,
    value: String,
}