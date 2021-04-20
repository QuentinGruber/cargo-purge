use std::env;
use std::fs;

fn main() {
    const USER_ENV: &str = env::consts::OS;
    let mut path: String = "unknown".to_string();
    let user_name: String = whoami::username();
    match USER_ENV {
        "linux" => path = "/usr/local/cargo/registry".to_string(),
        "windows" => path = format!("C:/Users/{}/.cargo/registry/", &user_name),
        _ => println!("Unsuported OS :( ..."),
    }
    if path != "unknown" {
        fs::remove_dir_all(path).unwrap();
        println!("cargo registery purged");
    }
}
