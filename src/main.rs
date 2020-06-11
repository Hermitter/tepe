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
                        .help("String to pass into a Telegram message"),
                ),
        )
        .get_matches();

    // Handle user input
    match matches.subcommand_name() {
        Some("test") => tepe.reply_chat_id().await,
        Some("send") => tepe.send(Some("text"), None).await,

        _ => {}
    }

    // println!("Value for config: {:?}", matches);
}
