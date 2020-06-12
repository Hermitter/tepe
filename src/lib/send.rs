use super::file_ext::{FileGroup, FILE_EXT_HASHMAP};
use super::TelegramBot;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use teloxide::types::InputFile;
use teloxide::{prelude::*, requests::Request};

impl TelegramBot {
    /// Send a document or text message
    // TODO: Use relevant Telegram API for specific media.
    pub async fn send(&self, message: Option<&str>, file_paths: &Vec<OsString>) {
        let mut requests = vec![];
        let message = message.unwrap_or("");

        match file_paths.len() {
            // text
            0 => {
                if message.len() > 0 {
                    self.send_text_message(message).await;
                }
            }
            // single file and a potential caption
            1 => {
                self.create_file_request(PathBuf::from(&file_paths[0]), message)
                    .send()
                    .await
                    .unwrap();
            }
            // multiple files and a potential message
            _ => {
                for file_path in file_paths {
                    self.create_file_request(PathBuf::from(file_path), "")
                        .send()
                        .await
                        .unwrap();
                }

                if message.len() > 0 {
                    self.send_text_message(message).await;
                }
            }
        }

        for file_path in file_paths {
            requests.push(12);
        }
    }

    /// Creates file specific Telegram requests for any file. Empty string captions are not sent to Telegram.
    fn create_file_request(
        &self,
        file: PathBuf,
        caption: &str,
    ) -> Box<dyn Request<Output = Message>> {
        let ext_name = file.extension().unwrap_or(OsStr::new(""));

        let file_group = FILE_EXT_HASHMAP
            .get(ext_name)
            .unwrap_or(&FileGroup::Document);

        // TODO: check if file exists
        let file = InputFile::file(file);

        match *file_group {
            FileGroup::Document => {
                return Box::new(
                    self.bot
                        .send_document(self.default_chat_id, file)
                        .caption(caption),
                );
            }
            FileGroup::Photo => {
                return Box::new(
                    self.bot
                        .send_photo(self.default_chat_id, file)
                        .caption(caption),
                );
            }
            FileGroup::Animation => {
                return Box::new(
                    self.bot
                        .send_animation(self.default_chat_id, file)
                        .caption(caption),
                );
            }
            FileGroup::Video => {
                return Box::new(
                    self.bot
                        .send_video(self.default_chat_id, file)
                        .caption(caption),
                );
            }
            FileGroup::Audio => {
                return Box::new(
                    self.bot
                        .send_audio(self.default_chat_id, file)
                        .caption(caption),
                );
            }
        }
    }
}
