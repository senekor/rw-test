use anyhow::Context;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
    Send { content: String },
    Receive,
}

/// send and receive joy with ✨ paekli-cli ✨
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

const SEND_MESSAGE: &str = "\
Thank you for trusting Paekli AG!
We will deliver your paekli in mint condition.";

const RECEIVE_MESSAGE: &str = "\
There aren't any paekli for you at the moment.
* tries to hide paekli on the storage shelf *";

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
        .context("the user's home directory seems to be corrupt")?;
    let storage_dir = project_dir.data_dir();
    std::fs::create_dir_all(storage_dir).context("failed to create storage directory")?;

    let paekli_path = storage_dir.join("content");

    match args.command {
        Command::Send { content } => {
            if std::fs::metadata(paekli_path).is_ok() {
                anyhow::bail!("cannot send paekli, storage is full!");
            }
            std::fs::write(storage_dir.join("content"), content)
                .context("failed to store paekli")?;
            println!("{SEND_MESSAGE}");
        }
        Command::Receive => println!("{RECEIVE_MESSAGE}"),
    }

    Ok(())
}
