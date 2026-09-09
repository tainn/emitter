use chrono::{Datelike, Local};
use std::env;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

struct Config {
    sleep_secs: u64,
    skip_days: String,
    repo_name: String,
    user_name: String,
    user_email: String,
    signing_key: String,
    gpg: String,
    repo_auth_url: String,
    branch: String,
}

impl Config {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let get = |key: &str| -> Result<String, Box<dyn std::error::Error>> {
            let val = env::var(key).map_err(|_| format!("missing environment variable: {key}"))?;
            if val.trim().is_empty() {
                return Err(format!("{key} must not be empty").into());
            }
            Ok(val)
        };

        let sleep_secs = get("SLEEP_SECS")?
            .parse::<u64>()
            .map_err(|_| "SLEEP_SECS must be a valid positive integer")?;

        Ok(Self {
            sleep_secs,
            skip_days: get("SKIP_DAYS")?,
            repo_name: get("REPO_NAME")?,
            user_name: get("USER_NAME")?,
            user_email: get("USER_EMAIL")?,
            signing_key: get("SIGNING_KEY")?,
            gpg: get("GPG")?,
            repo_auth_url: get("REPO_AUTH_URL")?,
            branch: get("BRANCH")?,
        })
    }
}

fn run_git(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git").args(args).status()?;
    if !status.success() {
        return Err(format!("git {args:?} exited with status: {status}").into());
    }
    Ok(())
}

fn init(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = env::temp_dir().join(&config.repo_name);

    fs::create_dir_all(&repo_path)?;
    env::set_current_dir(&repo_path)?;

    run_git(&["init"])?;
    run_git(&["config", "--local", "user.name", &config.user_name])?;
    run_git(&["config", "--local", "user.email", &config.user_email])?;
    run_git(&["config", "--local", "user.signingkey", &config.signing_key])?;
    run_git(&["config", "--local", "commit.gpgsign", &config.gpg])?;

    if let Err(e) = run_git(&["remote", "add", "origin", &config.repo_auth_url]) {
        eprintln!("note: git remote add failed (might already exist): {e}");
    }
    if let Err(e) = run_git(&["checkout", "-b", &config.branch]) {
        eprintln!("note: git checkout failed (branch might already exist): {e}");
    }

    Ok(())
}

fn repeat(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    run_git(&["pull", "origin", &config.branch])?;
    run_git(&["commit", "--allow-empty", "-m", "echo"])?;
    run_git(&["push", "origin", &config.branch])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    init(&config)?;

    loop {
        let now = Local::now();
        let weekday_num = now.weekday().num_days_from_monday().to_string();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S");

        if !config.skip_days.contains(&weekday_num) {
            if let Err(e) = repeat(&config) {
                eprintln!("warning: git operation failed: {e}");
            } else {
                println!("pushed at {timestamp}");
            }
        } else {
            let weekday_name = now.format("%A").to_string().to_lowercase();
            println!("skipped {weekday_name} at {timestamp}");
        }

        let sleep_secs = config.sleep_secs;
        println!("sleeping for {sleep_secs} secs at {timestamp}");

        thread::sleep(Duration::from_secs(config.sleep_secs));
    }
}
