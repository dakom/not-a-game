use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

pub static LOG_IDS: LazyLock<Mutex<HashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[macro_export]
macro_rules! log_once {
    ($id:expr, $($t:tt)*) => {{
        log_n!($id, 1, $($t)*);
    }};
}

#[macro_export]
macro_rules! log_n {
    ($id:expr, $max:expr, $($t:tt)*) => {{
        let mut lock = crate::logging::LOG_IDS.lock().unwrap();
        let count = lock.get($id).cloned().unwrap_or(0);
        if count < $max {
            log::info!($($t)*);
            lock.insert($id.to_string(), count+1);
        }
    }};
}
