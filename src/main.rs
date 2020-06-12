#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;
use clap::App;

pub mod lib;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    // Handle each command
    match app.subcommand() {
        ("test", Some(_sub_cmd)) => {
            let command = app.subcommand().1.unwrap();
            lib::TelegramBot::from(&command).reply_chat_id().await;
        }
        ("send", Some(_sub_cmd)) => {
            let command = app.subcommand().1.unwrap();
            let tepe = lib::TelegramBot::from(&command);

            let mut files = &vec![];
            let mut message = None;

            // get file paths
            if let Some(file_args) = command.args.get("files") {
                files = &file_args.vals;
            }

            // get message
            if let Some(message_arg) = command.args.get("message") {
                message = Some(message_arg.vals[0].to_str().expect("Invalid --message"));
            }

            tepe.send(message, files).await;
        }
        _ => {}
    };
}
