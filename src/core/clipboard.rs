use std::process::exit;

use arboard::{Clipboard, ImageData};
use tokio::time::{sleep, Duration};

use crate::core::store::Status;
use crate::utils::string_util::md5;

use super::database::{Database, Record};
use super::store::Store;

pub struct ClipboardWatcher;

pub struct ClipBoardOprator;

impl ClipBoardOprator {
    pub fn set_text(text: String) {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(text).unwrap();
    }
    pub fn _set_image(image: ImageData) {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_image(image).unwrap();
    }
}

impl ClipboardWatcher {
    pub async fn watch() {
        let mut clipboard = Clipboard::new().unwrap();
        let db = Database::new();
        let mut last_md5 = String::new();
        let mut count = 0;

        let config = Store::new().get_config();
        let max_record_count = config.max_record_count;
        let interval = config.interval;
        db.delete_over_limit(max_record_count).unwrap();

        loop {
            let status = Store::new().get_status();
            match status {
                Status::Open => {
                    let text = clipboard.get_text().unwrap();
                    let md5 = md5(&text);
                    if !text.trim().is_empty() && md5 != last_md5 {
                        db.insert_record(Record {
                            content: text,
                            md5: md5.clone(),
                            ..Default::default()
                        })
                        .unwrap();

                        if count > max_record_count {
                            db.delete_over_limit(max_record_count).unwrap();
                            count = 0;
                        } else {
                            count += 1;
                        }

                        last_md5 = md5;
                    }

                    sleep(Duration::from_millis(interval)).await;
                }
                Status::Closed => {
                    exit(1);
                }
            }
        }
    }
}
