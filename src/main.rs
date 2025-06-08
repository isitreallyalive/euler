use ::regex::Regex;
use clap::Parser;
use color_eyre::eyre::eyre;
use euler::{regex, Problem, PUBLIC_CHALLENGES};
use scraper::{Html, Selector};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    time::Duration,
};

#[macro_use]
extern crate clap;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scaffold a new problem
    New { n: usize },

    /// Runs an existing problem
    #[clap(aliases = &["r"])]
    Run { n: usize },

    /// Times all of the problems ran sequentially.
    All,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Cli { command } = Cli::parse();

    match command {
        Commands::New { n } => {
            // check whether the problem is public
            let public = n <= PUBLIC_CHALLENGES;

            // ensure that the problem exists
            let res = reqwest::blocking::get(format!("https://projecteuler.net/problem={}", n))?;
            let exists = res.url().path() != "/archives";

            if !exists {
                println!("Problem {} does not exist", n);
                return Ok(());
            }

            // fetch metadata
            let (title, description) = {
                let document = Html::parse_document(&res.text()?);
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
                        .replace(r#"\,"#, ",")
                        .replace(r#"\lt"#, "<")
                        .replace("^2", "²")
                };

                (
                    title.trim().to_string(),
                    description
                        .trim()
                        .lines()
                        .map(|line| format!("//! {}\n", line))
                        .collect::<String>(),
                )
            };

            // create the problem file
            let mut problem = File::create(
                std::env::current_dir()?
                    .join("src")
                    .join("problems")
                    .join(format!("p{}.rs", n)),
            )?;

            writeln!(
                problem,
                r#"//! Problem {}: {}
//!
{}
// time complexity: O(?)
use crate::prelude::*;

fn solve() -> Result<u32> {{
    unimplemented!();
}}

problem!({}, solve);"#,
                n, title, description, n
            )?;

            // if private, add to .gitignore
            if !public {
                let mut gitignore = OpenOptions::new().append(true).open(".gitignore")?;
                writeln!(gitignore, "src/problems/p{}.rs", n)?;
            }

            // todo: add to readme
        }

        Commands::Run { n } => {
            let problem = Problem::get(n).ok_or(eyre!("Problem not found"))?;
            let loops = problem.loops();
            let mut times = Vec::with_capacity(loops as usize);

            for i in 1..=loops {
                let (out, time) = problem.solve()?;
                times.push(time);
                if i == loops {
                    println!("{}", out);
                }
            }

            let total: Duration = times.iter().sum();
            let mean = total / loops as u32;
            println!("{} loops: Σ = {:?}, μ = {:?}", loops, total, mean);
        }
        Commands::All => {
            let problems = Problem::all();
            let loops: u32 = problems.iter().map(|p| p.loops() as u32).sum();
            let mut times = Vec::with_capacity(problems.len());
            for problem in problems {
                let (_, time) = problem.solve()?;
                times.push(time);
            }
            let total: Duration = times.iter().sum();
            let mean = total / loops;
            println!("Σ = {:?}, μ = {:?}", total, mean)
        }
    }

    Ok(())
}
