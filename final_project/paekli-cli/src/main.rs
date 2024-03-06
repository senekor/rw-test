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
    println!("Hello Silvia!");

    let args = Cli::parse();

    let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
        .context("the user's home directory seems to be corrupt")?;
    let storage_dir = project_dir.data_dir();
    std::fs::create_dir_all(storage_dir).context("failed to create storage directory")?;

    match args.command {
        Command::Send { content } => {
            let time = time::OffsetDateTime::now_utc().to_string();
            let paekli_path = storage_dir.join(time);

            if std::fs::metadata(&paekli_path).is_ok() {
                anyhow::bail!("Cannot send paekli, storage is full.");
            }
            std::fs::write(paekli_path, content).context("failed to store paekli")?;
            println!("{SEND_MESSAGE}");
        }
        Command::Receive => {
            let mut paekli: Vec<_> = std::fs::read_dir(storage_dir)
                .context("failed to read from storage")?
                .flatten()
                .map(|e| e.file_name())
                .collect();
            paekli.sort();
            let paekli_name = paekli
                .into_iter()
                .next()
                .context("There is no paekli to receive.")?
                .into_string()
                .ok()
                .context("paekli name should be a utf-8 string")?;

            let paekli_path = storage_dir.join(paekli_name);

            match std::fs::read_to_string(&paekli_path) {
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
            }
        }
    }

    Ok(())
}
