use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Config;

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config));
