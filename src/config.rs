use std::env;
use std::path::{Path, PathBuf};

const DEFAULT_HOME: &str = ".config/alt";
const DEFAULT_SHIM_DIR: &str = ".local/alt/shims";

pub fn home_dir() -> PathBuf {
    match env::var("ALT_HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(DEFAULT_HOME)
        }
    }
}

pub fn shim_dir() -> PathBuf {
    env::var("ALT_SHIM_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(DEFAULT_SHIM_DIR)
        })
}

#[cfg(test)]
mod tests {
    use crate::config;
    use std::env;
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn home_dir_should_default() {
        let res = {
            let _guard = ENV_MUTEX.lock().unwrap();
            env::remove_var("ALT_HOME");
            config::home_dir()
        };

        assert_eq!(
            res,
            Path::new(&env::var("HOME").unwrap()).join(config::DEFAULT_HOME)
        );
    }

    #[test]
    fn home_dir_should_read_alt_home_env() {
        let res = {
            let _guard = ENV_MUTEX.lock().unwrap();
            env::set_var("ALT_HOME", "/path/to/phony/home");
            config::home_dir()
        };

        assert_eq!(res, PathBuf::from("/path/to/phony/home"));
    }
}
