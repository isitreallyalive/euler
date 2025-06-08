use clap::Parser;
use color_eyre::Result;
use euler::Problem;
use owo_colors::OwoColorize;
use std::{fs::OpenOptions, io::Write};

mod new;
mod run;

// this needs to be here to make sure all of the solutions get registered to the inventory
extern crate problems;

/// How many solutions can be shared publicly according to Project Euler's website.
const PUBLIC_CHALLENGES: usize = 100;

#[macro_use]
extern crate clap;

#[derive(Parser)]
struct Cli {
    /// Problem number to run or create
    n: Option<usize>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let Cli { n } = Cli::parse();

    if let Some(n) = n {
        match Problem::get(n) {
            Some(problem) => {
                // run the problem
                let (out, times) = run::run(problem)?;
                let (total, mean, sd) = run::summarise(&times, problem.loops as u32);

                println!(
                    r#"{}
Solution: {}

Total time: {total:?}
Mean time: {mean:?} / loop
Std dev: {sd:?}
Ran for: {} loops"#,
                    hyperlink(
                        format!("https://projecteuler.net/problem={n}"),
                        format!("Problem {n}").bold().green().to_string()
                    ),
                    out.bold(),
                    problem.loops
                );
            }
            None => {
                // create the problem
                if let Some((title, description)) = new::fetch(n) {
                    new::write(n, title, description)?;

                    // if the challenge is private, add it to the gitignore
                    let public = n <= PUBLIC_CHALLENGES;
                    if !public {
                        let mut gitignore = OpenOptions::new()
                            .append(true)
                            .open("problems/.gitignore")?;
                        writeln!(gitignore, "\nsrc/p{}.rs", n)?;
                    }

                    // todo: add to readme
                } else {
                    println!(
                        r#"{}

Problem {} does not exist.
You can find a full list of existing problems {}."#,
                        "Error!".red().bold(),
                        format!("#{n}").bold(),
                        hyperlink(
                            "https://projecteuler.net/archives".to_string(),
                            "here".to_string()
                        )
                        .bold()
                    )
                }
            }
        }
    } else {
        // run all problems
        let problems = Problem::all();
        let mut all_times = Vec::new();
        let mut total_loops = 0;

        for problem in problems {
            let (out, times) = run::run(problem)?;
            let (total, mean, sd) = run::summarise(&times, problem.loops as u32);

            // add to overall statistics
            all_times.extend(times);
            total_loops += problem.loops;
        }
    }

    Ok(())
}

fn problem_url(n: usize) -> String {
    format!("https://projecteuler.net/problem={}", n)
}

/// Make a URL clickable using ANSI codes
fn hyperlink(url: String, text: String) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}
