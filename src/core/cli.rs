use chrono::{Local, TimeZone};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use std::process::exit;

use unicode_width::UnicodeWidthStr;

use crate::utils::string_util::unicode_width_splice;

use super::store::Store;
use super::tcp_manager::TcpManager;
use super::{
    clipboard::{ClipBoardOprator, ClipboardWatcher},
    database::{Database, Record},
};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// watch clipboard
    Watch,
    /// select <KEY>
    Select { search: Option<String> },
    /// select history
    History,
    /// stop watch
    Stop,
}

pub struct Cli {
    args: Args,
}

fn records_to_select_items(records: &Vec<Record>) -> Vec<String> {
    let mut items = vec![];
    // 单选序号
    let mut number = 1;
    records.clone().into_iter().for_each(|record| {
        let content = record.content.replace("\n", "");
        let content = content.trim();
        let item = if UnicodeWidthStr::width(content) > 50 {
            unicode_width_splice(&content, 50)
        } else {
            // 时间右对齐
            let space = " ".repeat(50 - UnicodeWidthStr::width(content));
            format!("{}{}", content, space)
        };
        let dt = Local
            .timestamp_millis_opt(record.create_time as i64)
            .unwrap();
        let create_time = dt.format("%Y-%m-%d %H:%M:%S").to_string();
        // 添加序号
        let item = if number < 10 {
            format!("(0{}) {} {}", number, item, create_time)
        } else {
            format!("({}) {} {}", number, item, create_time)
        };
        number += 1;
        items.push(item);
    });
    items
}

impl Cli {
    pub fn new() -> Self {
        Self {
            args: Args::parse(),
        }
    }
    pub async fn run(&self) {
        match &self.args.cmd {
            Commands::Watch => {
                tokio::spawn(async {
                    if let Err(err) = TcpManager::start().await {
                        println!("{}", err.to_string());
                        // 退出程序
                        std::process::exit(1);
                    }
                });
                ClipboardWatcher::watch().await;
            }
            Commands::Select { search } => {
                let records = match search {
                    Some(key) => Database::new().find(&key).unwrap(),
                    None => {
                        let limit = Store::new().get_config().limit;
                        Database::new().find_by_limit(limit).unwrap()
                    }
                };

                if records.is_empty() {
                    println!("record was not found");
                    exit(1);
                }

                let items = records_to_select_items(&records);
                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("What do you choose?({})", records.len()))
                    .default(0)
                    .items(&items)
                    .interact_opt()
                    .unwrap();
                match selection {
                    Some(index) => {
                        let record = records[index].clone();
                        ClipBoardOprator::set_text(record.content);
                    }
                    None => {
                        exit(1);
                    }
                };
            }
            Commands::History => {
                let records = Database::new().find_all().unwrap();

                if records.is_empty() {
                    println!("record was not found");
                    exit(1);
                }

                let items = records_to_select_items(&records);
                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("What do you choose?({})", records.len()))
                    .default(0)
                    .items(&items)
                    .interact_opt()
                    .unwrap();
                match selection {
                    Some(index) => {
                        let record = records[index].clone();
                        ClipBoardOprator::set_text(record.content);
                    }
                    None => {
                        exit(1);
                    }
                };
            }
            Commands::Stop => {
                if let Err(_) = TcpManager::stop().await {
                    println!("Poem not starting");
                }
            }
        }
    }
}
