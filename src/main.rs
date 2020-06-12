#[macro_use]
extern crate lazy_static;
use clap::{App, Arg, SubCommand};
pub mod lib;

#[tokio::main]
async fn main() {
    // TODO: add option to manually add bot_token and chat_id
    let tepe = lib::TelegramBot::new();

    // Define CLI
    let matches = App::new("Tepe")
        .version("0.1")
        .author("Hermitter")
        .about("Send messages and files through a telegram bot.")
        .subcommand(
            SubCommand::with_name("test")
                .about("Test to check that the bot is properly working. Once messaged, the bot will respond reply with the chat_id")
        )
        .subcommand(
            SubCommand::with_name("send")
                .arg(Arg::with_name("files").required(false).multiple(true))
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("--message")
                        .takes_value(true)
                        .help("String to pass into a Telegram message"),
                ),
        )
        .get_matches();

    // Handle each command
    match matches.subcommand_name() {
        Some("test") => tepe.reply_chat_id().await,
        Some("send") => {
            let command = matches.subcommand().1.unwrap();
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

            tepe.send(message, files).await
        }
        // Some("prompt") => {}, // TODO
        _ => {}
    }
}
