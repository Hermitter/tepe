use super::file_ext::{FileGroup, FILE_EXT_HASHMAP};
use super::Error;
use super::TelegramBot;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use teloxide::types::InputFile;
use teloxide::{prelude::*, requests::RequestWithFile};

impl TelegramBot {
    /// Send a document or text message
    pub async fn send(
        &self,
        message: Option<&str>,
        file_paths: &Vec<OsString>,
    ) -> Result<(), Error> {
        if self.chat_ids.is_empty() {
            return Err(Error::MissingChatId);
        }

        if file_paths.is_empty() && message.is_none() {
            return Err(Error::NoInput);
        }

        let message = message.unwrap_or("");

        for chat_id in &self.chat_ids {
            match file_paths.len() {
                // text message
                0 => {
                    if message.len() > 0 {
                        self.send_text_message(message, *chat_id).await?;
                    }
                }
                // single file and an optional text caption
                1 => {
                    self.create_file_request(*chat_id, PathBuf::from(&file_paths[0]), message)?
                        .send()
                        .await??;
                }
                // multiple files and an optional text message
                _ => {
                    for file_path in file_paths {
                        self.create_file_request(*chat_id, PathBuf::from(file_path), "")?
                            .send()
                            .await??;
                    }

                    if message.len() > 0 {
                        self.send_text_message(message, *chat_id).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Creates file specific Telegram requests for any file. Empty string captions are not sent to Telegram.
    fn create_file_request(
        &self,
        chat_id: i64,
        file: PathBuf,
        caption: &str,
    ) -> Result<Box<dyn RequestWithFile<Output = Message>>, Error> {
        let ext_name = file.extension().unwrap_or(OsStr::new(""));

        if !file.is_file() {
            return Err(Error::FileNotFound {
                path: String::from(format!("{:?}", file)),
            });
        };

        // set file group to document, if nothing is found
        let file_group = FILE_EXT_HASHMAP
            .get(ext_name)
            .unwrap_or(&FileGroup::Document);

        let file = InputFile::file(file);

        match *file_group {
            FileGroup::Document => {
                return Ok(Box::new(
                    self.bot.send_document(chat_id, file).caption(caption),
                ));
            }
            FileGroup::Photo => {
                return Ok(Box::new(
                    self.bot.send_photo(chat_id, file).caption(caption),
                ));
            }
            FileGroup::Animation => {
                return Ok(Box::new(
                    self.bot.send_animation(chat_id, file).caption(caption),
                ));
            }
            FileGroup::Video => {
                return Ok(Box::new(
                    self.bot.send_video(chat_id, file).caption(caption),
                ));
            }
            FileGroup::Audio => {
                return Ok(Box::new(
                    self.bot.send_audio(chat_id, file).caption(caption),
                ));
            }
        }
    }
}
