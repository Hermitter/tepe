use std::sync::Arc;
use teloxide::{prelude::*, utils::markdown};

pub struct TelegramBot {
    /// Teleoxide representation of a Telegram bot
    pub bot: Arc<Bot>,

    /// Default destination for messages
    pub default_chat_id: u64,
}

impl TelegramBot {
    /// Instantiate a Telegram bot from environment variables.
    pub fn new() -> TelegramBot {
        let default_chat_id = std::env::var("TEPE_TELEGRAM_CHAT_ID")
            .expect("TEPE_TELEGRAM_CHAT_ID has not been set!")
            .parse::<u64>()
            .expect("Invalid format for TEPE_TELEGRAM_CHAT_ID");

        let token = std::env::var("TEPE_TELEGRAM_BOT_TOKEN")
            .expect("TEPE_TELEGRAM_BOT_TOKEN has not been set!");

        TelegramBot::from(&token, default_chat_id)
    }

    /// Instantiate a Telegram bot from argument variables.
    pub fn from(token: &str, default_chat_id: u64) -> TelegramBot {
        TelegramBot {
            default_chat_id,
            bot: Bot::new(
                std::env::var("TEPE_TELEGRAM_BOT_TOKEN")
                    .expect("TEPE_TELEGRAM_BOT_TOKEN has not been set!")
                    .as_str(),
            ),
        }
    }

    /// A one time response to a chat with the current chat_id
    pub async fn reply_chat_id(&self) {
        println!("*********************************************************************\nYour Telegram bot is now running! Try sending a message.\nOn success, the chat_id is printed.");

        Dispatcher::new(self.bot.clone())
            .messages_handler(|rx: DispatcherHandlerRx<Message>| {
                rx.for_each(|message| async move {
                    let response = format!(
                        "{} `{}`",
                        markdown::escape("This bot is good to go! This chat_id is"),
                        &message.chat_id().to_string()
                    );

                    message
                        .answer(response)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .send()
                        .await
                        .log_on_error()
                        .await;

                    println!("{}", format!("\nSuccessful reply from chat_id: {}\n*********************************************************************", &message.chat_id()));
                    std::process::exit(0);
                })
            })
            .dispatch()
            .await;
    }

    pub async fn send(&self, text: Option<&str>, files_paths: Option<&[&str]>) {
        // self.bot.send_message(self.default_chat_id, text).send().await.unwrap();
    }
}
