use ::regex::Regex;
use clap::Parser;
use color_eyre::eyre::eyre;
use euler::{prelude::*, regex, PUBLIC_CHALLENGES};
use scraper::{Html, Selector};
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::time::Instant;

mod solutions;

#[macro_use]
extern crate clap;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scaffold a new solution
    New { problem: usize },

    /// Runs an existing solution
    Run { problem: usize },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let Cli { command } = Cli::parse();

    match command {
        Commands::New { problem } => {
            // check whether the problem is public
            let public = problem <= PUBLIC_CHALLENGES;

            // ensure that the problem exists
            let res = reqwest::get(format!("https://projecteuler.net/problem={}", problem)).await?;
            let exists = res.url().path() != "/archives";

            if !exists {
                println!("Problem {} does not exist", problem);
                return Ok(());
            }

            // fetch metadata
            let (title, description) = {
                let document = Html::parse_document(&res.text().await?);
                let title = {
                    let selector = Selector::parse("h2").unwrap();

                    document
                        .select(&selector)
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>()
                };
                let description = {
                    let selector = Selector::parse(".problem_content").unwrap();
                    let regex = Regex::new(regex::LATEX)?;

                    regex
                        .replace_all(
                            &document
                                .select(&selector)
                                .next()
                                .unwrap()
                                .text()
                                .collect::<String>(),
                            "$1",
                        )
                        .to_string()
                        .replace(r#"\dots"#, "...")
                        .replace(r#"\times"#, "×")
                };

                (
                    title.trim().to_string(),
                    description
                        .trim()
                        .lines()
                        .map(|line| format!("// {}\n", line))
                        .collect::<String>(),
                )
            };

            // create the solution file
            let mut solution = File::create(
                std::env::current_dir()?
                    .join("src")
                    .join("solutions")
                    .join(format!("p{}.rs", problem)),
            )?;

            writeln!(
                solution,
                r#"// Problem {}: {}
//
{}
use euler::prelude::*;

pub const LOOPS: u8 = 100;
pub struct Problem;

impl Execute for Problem {{
    fn execute(&self) -> Result<Return> {{
        let value = unimplemented!();

        Ok(Return::None)
    }}
}}"#,
                problem, title, description
            )?;

            // add to solution list
            let mut contents = read_to_string("src/solutions/mod.rs")?;
            let start = contents.rfind(",").unwrap() + 1;
            contents.insert_str(start, &format!("\n\t{},", problem));

            let mut solutions = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("src/solutions/mod.rs")?;

            solutions.write_all(contents.as_bytes())?;

            // if private, add to .gitignore
            if !public {
                let mut gitignore = OpenOptions::new().append(true).open(".gitignore")?;
                writeln!(gitignore, "src/solutions/p{}.rs", problem)?;
            }

            // todo: add to readme
        }

        Commands::Run { problem } => {
            let solution = solutions::get(problem).ok_or(eyre!("Problem not found"))?;
            let loops = solutions::loops(problem);

            let start = Instant::now();
            let out = solution.execute()?;

            for _ in 0..(loops - 1) {
                solution.execute()?;
            }

            let time = start.elapsed();
            println!("{} loops in {:?}", loops, time);

            match out {
                Return::None => {}
                Return::u32(n) => println!("{}", n),
                Return::u64(n) => println!("{}", n),
                Return::i32(n) => println!("{}", n),
            }
        }
    }

    Ok(())
}
