use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    Send {
        content: String,
        #[arg(long("to"))]
        receiver: String,
        #[arg(long)]
        express: bool,
    },
    Receive {
        #[arg(long("for"))]
        receiver: String,
    },
}

/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

const SEND_MESSAGE: &str = "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.";

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
        .context("the user's home directory seems to be corrupt")?;
    let storage_dir = project_dir.data_dir();

    match args.command {
        Command::Send {
            content,
            receiver,
            express,
        } => {
            let mut receiver_dir = storage_dir.join(receiver);
            if express {
                receiver_dir.push("express")
            }
            let receiver_dir = receiver_dir;

            fs::create_dir_all(&receiver_dir).context("failed to create receiver directory")?;

            let time = time::OffsetDateTime::now_utc().to_string();
            let paekli_path = receiver_dir.join(time);

            if fs::metadata(&paekli_path).is_ok() {
                anyhow::bail!("Cannot send paekli, storage is full.");
            }
            fs::write(paekli_path, content).context("failed to store paekli")?;
            println!("{SEND_MESSAGE}");
        }
        Command::Receive { receiver } => {
            let receiver_dir = storage_dir.join(&receiver);
            let express_dir = receiver_dir.join("express");

            let paekli_path = get_first_paekli_path_in_dir(&express_dir)
                .or_else(|| get_first_paekli_path_in_dir(&receiver_dir))
                .context(format!("There is no paekli for {}.", receiver))?;

            match fs::read_to_string(&paekli_path) {
                Ok(content) => {
                    fs::remove_file(paekli_path)
                        .context("failed to remove received paekli from storage")?;
                    println!("Here is your paekli:\n{content}");
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        anyhow::bail!("There is no paekli to receive.");
                    }
                    _ => return Err(e).context("failed to read from paekli storage"),
                },
            }
        }
    }

    Ok(())
}

fn get_first_paekli_path_in_dir(dir: &Path) -> Option<PathBuf> {
    let mut paekli: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.metadata().is_ok_and(|m| m.is_file()))
        .map(|e| e.file_name())
        .collect();
    paekli.sort();
    paekli
        .into_iter()
        .next()?
        .into_string()
        .ok()
        .map(|name| dir.join(name))
}
