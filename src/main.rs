#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate clap;
use clap::App;
use std::ffi::OsString;
pub mod lib;
use lib::error::Error;

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|error| error.exit());
}

async fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    // Handle each command
    match app.subcommand() {
        ("test", Some(sub_cmd)) => {
            lib::TelegramBot::from_clap(sub_cmd)?
                .reply_chat_id()
                .await?;
        }

        ("send", Some(sub_cmd)) => {
            let tepe = lib::TelegramBot::from_clap(&sub_cmd)?;

            let mut files = &Vec::<OsString>::new();
            let mut message = None;

            // get file paths
            if let Some(file_args) = sub_cmd.args.get("files") {
                files = &file_args.vals;
            }

            // get message
            if let Some(message_arg) = sub_cmd.args.get("message") {
                message = message_arg.vals[0].to_str();
            }

            tepe.send(message, files).await?;
        }
        _ => return Err(Error::NoInput),
    };

    Ok(())
}
