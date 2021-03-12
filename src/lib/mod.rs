use clap::ArgMatches;
use teloxide::{prelude::*, requests::Request, utils::markdown, BotBuilder};
pub mod error;
use error::{CliExit, Error};
pub mod file_ext;
pub mod send;

pub struct TelegramBot {
    /// Teleoxide representation of a Telegram bot
    pub bot: Bot,

    /// Default destination for messages
    pub chat_ids: Vec<i64>,
}

impl TelegramBot {
    /// Instantiate a Telegram bot from CLAP flags or default to environment variables.
    // TODO: change `cli_expect` to something that returns `crate::Error`
    pub fn from_clap(command: &ArgMatches) -> Result<TelegramBot, Error> {
        let mut chat_ids = Vec::<i64>::new();

        // get chat_ids from flags
        if let Some(args) = command.args.get("chat_ids") {
            for id in args.vals.iter() {
                chat_ids.push(
                    id.clone()
                        .into_string()
                        .cli_expect(&format!("\nError parsing chat_id:\n\t{:?}", id))
                        .trim()
                        .parse::<i64>()
                        .cli_expect(&format!("\nError parsing chat_id:\n\t{:?}", id)),
                );
            }
        }

        // get chat_id from environment variable
        if let Some(default_chat_id) = std::env::var("TEPE_TELEGRAM_CHAT_ID").ok() {
            chat_ids.push(
                default_chat_id
                    .trim()
                    .parse::<i64>()
                    .cli_expect("\nError parsing TEPE_TELEGRAM_CHAT_ID"),
            );
        }

        // token from flag or environment variable.
        let token = match command.args.get("token") {
            Some(arg) => arg.vals[0]
                .clone()
                .into_string()
                .cli_expect("\nError reading (--token, -t) argument"),
            None => std::env::var("TEPE_TELEGRAM_BOT_TOKEN")
                .cli_expect("\nTEPE_TELEGRAM_BOT_TOKEN has not been set"),
        };

        Ok(TelegramBot {
            bot: Bot::builder().token(token).build(),
            chat_ids: chat_ids,
        })
    }

    /// Print and send the Telegram `chat_id` to the first user response.
    pub async fn reply_chat_id(self) -> Result<(), Error> {
        println!("*********************************************************************\nYour Telegram bot is now running! Try sending it a message on Telegram.\nOn success, the chat_id is printed.\n\nPress Ctrl+c to exit.");

        Dispatcher::new(self.bot)
            .messages_handler(|rx: DispatcherHandlerRx<Message>| {
                rx.for_each(|message| async move {
                    let response = format!(
                        "{} `{}`",
                        markdown::escape("This bot is good to go! This chat_id is"),
                        &message.chat_id().to_string()
                    );

                    let request = message
                        .answer(response)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .send()
                        .await;

                    // exit and print a success or error message
                    match request {
                        Ok(message) => {
                            println!("{}", format!("\nSuccessful reply from chat_id: {}\n*********************************************************************", &message.chat_id()));
                        }
                        Err(error) => {Error::from(error).exit()}
                    }
                })
            })
            .dispatch()
            .await;

        Ok(())
    }

    /// Send a text message to the `chat_id`.
    pub async fn send_text_message(&self, text: &str, chat_id: i64) -> Result<(), Error> {
        self.bot.send_message(chat_id, text).send().await?;
        Ok(())
    }
}
