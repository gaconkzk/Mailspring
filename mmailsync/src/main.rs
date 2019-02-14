use std::env;
use serde_json::json;
use serde::{ Serialize, Deserialize };
use std::io::stdin;
use std::io::stdout;
use std::io::{ Write, Read };
use std::io::Error;
use clap::{ Arg, App, SubCommand };
use clap::{ crate_authors, crate_description, crate_version };
use mmailsync::core::MMailSync;

#[derive(Serialize, Deserialize)]
struct SyncModel {
    #[serde(rename = "type")]
    model_type: String,
    #[serde(rename = "modelJSONs")]
    model_json: Vec<String>,
    #[serde(rename = "modelClass")]
    model_class: String,
}

fn main() {
    let matches = App::new("Mega Mail Sync")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(Arg::with_name("mode")
            .long("mode")
            .value_name("MODE")
            .help("startup mode")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("info")
            .long("info")
            .value_name("ACCOUNT_INFO")
            .help("the account email")
            .takes_value(true)
        )
        .get_matches();

    let mode = matches.value_of("mode").expect("Crazy bug!!!");
    let account = matches.value_of("info").unwrap_or("unknown@unknown");

    let null:Option<Error> = None;

    match mode {
        "migrate" => print!("{}",
                              json!({
                              "error": null,
                              }).to_string()),
        "sync" => {
            let mut ms = MMailSync::new();
            ms.start(account);
        },
        other => print!("{}", json!({"msg": "Unknown mode"}).to_string()),
    };
}
