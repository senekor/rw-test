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
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.";

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
                anyhow::bail!("Cannot send paekli, storage is full.");
            }
            std::fs::write(storage_dir.join("content"), content)
                .context("failed to store paekli")?;
            println!("{SEND_MESSAGE}");
        }
        Command::Receive => match std::fs::read_to_string(&paekli_path) {
            Ok(content) => {
                std::fs::remove_file(paekli_path)
                    .context("failed to remove received paekli from storage")?;
                println!("Here is your paekli:\n{content}");
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    anyhow::bail!("There is no paekli to receive.");
                }
                _ => return Err(e).context("failed to read from paekli storage"),
            },
        },
    }

    Ok(())
}
