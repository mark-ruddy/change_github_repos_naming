use clap::Parser;
use dotenv::dotenv;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, io::Write};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Format {
    KebabToSnake,
    SnakeToKebab,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    username: String,
    /// Options are kebab-to-snake and snake-to-kebab
    #[clap(short, long)]
    format: String,
    /// No check for yes/no confirmation before renaming
    #[clap(short, long, default_value = "true")]
    interactive: bool,
}

#[derive(Serialize, Deserialize)]
struct Repo {
    name: String,
}

fn format_arg_to_enum(format: &str) -> Result<Format, Box<dyn Error>> {
    match format {
        "kebab-to-snake" => Ok(Format::KebabToSnake),
        "snake-to-kebab" => Ok(Format::SnakeToKebab),
        _ => return Err("Invalid format, options are kebab-to-snake or snake-to-kebab".into()),
    }
}

fn format_name(name: &str, format: &Format) -> String {
    match format {
        Format::KebabToSnake => name.replace("-", "_"),
        Format::SnakeToKebab => name.replace("_", "-"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();
    let format = format_arg_to_enum(&args.format)?;

    // get API token either from environment variable or .env file
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN")?;
    println!("token is: {}", token);

    let client = Client::new();
    let request_url = format!("https://api.github.com/user/repos?per_page=100");

    let resp = client
        .get(&request_url)
        .header("User-Agent", "rust")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let repo_names = resp.json::<Vec<Repo>>().await?;

    /*
    for repo in repo_names {
        println!("repo name: {}", repo.name);
    }
    */

    for repo in repo_names {
        let updated_name = format_name(&repo.name, &format);
        if updated_name == repo.name {
            println!("Skipping repo {}, no change after formatting", repo.name);
            continue;
        }

        if args.interactive {
            print!(
                "Renaming repo {} to {}, continue? [y/n] ",
                repo.name, updated_name
            );
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input != "Y" && input != "y" && input != "yes" {
                println!("Skipping {}", repo.name);
                continue;
            }
        } else {
            println!("Renaming repo {} to {}", repo.name, updated_name);
        }

        let request_url = format!(
            "https://api.github.com/repos/{}/{}",
            args.username, repo.name
        );
        let response = client
            .patch(&request_url)
            .header("User-Agent", "rust")
            .header("Authorization", format!("Bearer {}", token))
            .json(&Repo {
                name: updated_name.clone(),
            })
            .send()
            .await?;
        if response.status() == StatusCode::OK {
            println!(
                "Successfully renamed repo {} to {}",
                repo.name, updated_name
            );
        } else {
            let body = response.text().await?;
            println!(
                "Failed to rename repo {}, server response: {}",
                repo.name, body
            );
        }
    }

    Ok(())
}
