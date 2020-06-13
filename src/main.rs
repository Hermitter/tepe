#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;
use clap::App;
use std::ffi::OsString;

pub mod error;
use error::Error;
pub mod lib;

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|error| error.exit());
}

async fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    // Handle each command
    // TODO: use _sub_cmd instead of app.subcommand
    match app.subcommand() {
        ("test", Some(_sub_cmd)) => {
            let cmd = app.subcommand().1.unwrap();
            lib::TelegramBot::from_clap(&cmd)?.reply_chat_id().await?;
        }

        ("send", Some(_sub_cmd)) => {
            let cmd = app.subcommand().1.unwrap();
            let tepe = lib::TelegramBot::from_clap(&cmd)?;

            let mut files = &Vec::<OsString>::new();
            let mut message = None;

            // get file paths
            if let Some(file_args) = cmd.args.get("files") {
                files = &file_args.vals;
            }

            // get message
            if let Some(message_arg) = cmd.args.get("message") {
                message = message_arg.vals[0].to_str();
            }

            tepe.send(message, files).await;
        }
        _ => {}
    };

    Ok(())
}
