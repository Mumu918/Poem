use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};


use super::config::Config;

#[derive(Debug, Default, Clone)]
pub enum Status {
    #[default]
    Open,
    Closed,
}

#[derive(Debug, Default, Clone)]
pub struct Value {
    pub status: Status,
    pub config: Config,
}

#[derive(Debug)]
pub struct Store {
    pub value: Arc<Mutex<Value>>,
}

impl Store {
    pub fn new() -> &'static Store {
        static STORE: OnceCell<Store> = OnceCell::new();
        STORE.get_or_init(|| Store {
            value: Arc::new(Mutex::new(Value::default())),
        })
    }

    pub fn set_status(&self, status: Status) {
        self.value.lock().unwrap().status = status;
    }

    pub fn get_status(&self) -> Status {
        self.value.lock().unwrap().status.clone()
    }

    pub fn get_config(&self) -> Config {
        self.value.lock().unwrap().config.clone()
    }
}
