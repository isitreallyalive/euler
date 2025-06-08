use clap::Parser;
use color_eyre::Result;
use color_eyre::eyre::eyre;
use euler::Problem;
use owo_colors::OwoColorize;
use regex::Regex;
use scraper::{Html, Selector};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::Write,
    time::Duration,
};

mod run;

// this needs to be here to make sure all of the solutions get registered to the inventory
extern crate problems;

/// How many solutions can be shared publicly according to Project Euler's website.
const PUBLIC_CHALLENGES: usize = 100;

/// LaTeX regex
const LATEX: &'static str = r#"\$\$?([^$]+)\$?\$"#;

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
                    url(
                        format!("https://projecteuler.net/problem={n}"),
                        format!("Problem {n}").bold().green().to_string()
                    ),
                    out.bold(),
                    problem.loops
                );
            }
            None => {
                // create the problem
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

/// Make a URL clickable using ANSI codes
fn url(url: String, text: String) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}

// fn handle_command(command: Command) -> Result<()> {
//     match command {
//         Command::New { n } => {
//             // check whether the problem is public
//             let public = n <= PUBLIC_CHALLENGES;

//             // ensure that the problem exists
//             let res = reqwest::blocking::get(format!("https://projecteuler.net/problem={}", n))?;
//             let exists = res.url().path() != "/archives";

//             if !exists {
//                 println!("Problem {} does not exist", n);
//                 return Ok(());
//             }

//             // fetch metadata
//             let (title, description) = {
//                 let document = Html::parse_document(&res.text()?);
//                 let title = {
//                     let selector = Selector::parse("h2").unwrap();

//                     document
//                         .select(&selector)
//                         .next()
//                         .unwrap()
//                         .text()
//                         .collect::<String>()
//                 };
//                 let description = {
//                     let selector = Selector::parse(".problem_content").unwrap();
//                     let regex = Regex::new(LATEX)?;

//                     regex
//                         .replace_all(
//                             &document
//                                 .select(&selector)
//                                 .next()
//                                 .unwrap()
//                                 .text()
//                                 .collect::<String>(),
//                             "$1",
//                         )
//                         .to_string()
//                         .replace(r#"\dots"#, "...")
//                         .replace(r#"\times"#, "×")
//                         .replace(r#"\,"#, ",")
//                         .replace(r#"\lt"#, "<")
//                         .replace("^2", "²")
//                 };

//                 (
//                     title.trim().to_string(),
//                     description
//                         .trim()
//                         .lines()
//                         .map(|line| format!("//! {}\n", line))
//                         .collect::<String>(),
//                 )
//             };

//             // create the problem file
//             let mut problem = File::create(
//                 std::env::current_dir()?
//                     .join("problems")
//                     .join("src")
//                     .join(format!("p{}.rs", n)),
//             )?;

//             writeln!(
//                 problem,
//                 r#"//! Problem {}: {}
// //!
// {}
// // time complexity: O(?)
// use euler::prelude::*;

// fn solve() -> Result<u32> {{
//     unimplemented!();
// }}

// problem!({}, solve);"#,
//                 n, title, description, n
//             )?;

//             // if private, add to .gitignore
//             if !public {
//                 let mut gitignore = OpenOptions::new().append(true).open(".gitignore")?;
//                 writeln!(gitignore, "src/problems/p{}.rs", n)?;
//             }

//             // todo: add to readme
//         }

//         Command::Run { n } => {}

//         Command::All => {
//             let problems = Problem::all();
//             let loops: usize = problems.iter().map(|p| p.loops).sum();
//             let mut times = Vec::with_capacity(problems.len());
//             for problem in problems {
//                 let (_, time) = problem.solve()?;
//                 times.push(time);
//             }
//             let total: Duration = times.iter().sum();
//             let mean = total / loops as u32;
//             println!("Σ = {:?}, μ = {:?}", total, mean)
//         }
//     }

//     Ok(())
// }
