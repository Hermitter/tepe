use clap::{App, Arg, SubCommand};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // let args = Cli::from_args();
    // send_message(&args.message).await;

    let matches = App::new("Tepe")
        .version("1.0")
        .author("Hermitter")
        .about("Send messages and files through a telegram bot.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Hermitter")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {:?}", matches);
}

async fn send_message(text: &str) {
    let bot = Bot::new("ADD_BOT_TOKEN_HERE");
    bot.send_message(826526167, text).send().await.unwrap();
}
