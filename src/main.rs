#[macro_use]
extern crate lazy_static;
use clap::{App, Arg, ArgMatches, SubCommand};
pub mod lib;

#[tokio::main]
async fn main() {
    // Define CLI
    let app = App::new("Tepe")
        .version("0.1")
        .author("Hermitter")
        .about("Send messages and files through a telegram bot.")
        .subcommand(
    SubCommand::with_name("test")
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("--token")
                        .takes_value(true)
                        .help("Telegram bot token."),
                )
                .about("Test to check that the bot is properly working. Once messaged, the bot will respond reply with the chat_id")
        )
        .subcommand(
    SubCommand::with_name("send")
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("--token")
                        .max_values(1)
                        .takes_value(true)
                        .help("Telegram bot token."),
                )
                .arg(
                    Arg::with_name("chat_ids")
                        .short("c")
                        .long("--chat-id")
                        .multiple(true)
                        .takes_value(true)
                        .help("Telegram bot token."),
                )
                .arg(Arg::with_name("files").required(false).multiple(true))
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("--message")
                        .takes_value(true)
                        .help("String to pass into a Telegram message."),
            ),
        )
        .get_matches();

    // Handle each command
    match app.subcommand() {
        ("test", Some(sub_cmd)) => {
            let command = app.subcommand().1.unwrap();
            lib::TelegramBot::from(&command).reply_chat_id().await;
        }
        ("send", Some(sub_cmd)) => {
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
