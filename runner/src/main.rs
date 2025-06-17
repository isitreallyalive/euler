use clap::Parser;
use cli_table::WithTitle;
use color_eyre::Result;
use euler::Problem;
use owo_colors::OwoColorize;
use std::{fs::OpenOptions, io::Write, time::Duration};

mod all;
mod new;
mod run;

// this needs to be here to make sure all of the solutions get registered to the inventory
extern crate problems;

/// How many solutions can be shared publicly according to Project Euler's website.
const PUBLIC_CHALLENGES: usize = 100;

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
                let (out, times, correct) = run::run(problem)?;

                println!(
                    r#"{}
Solution: {}
"#,
                    hyperlink(
                        format!("https://projecteuler.net/problem={n}"),
                        format!("Problem {n}").bold().green().to_string()
                    ),
                    format_solution(out, correct).bold(),
                );
                print_summary(times, problem.loops);
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
        let mut problems = Problem::all();
        problems.sort_by(|a, b| a.n.cmp(&b.n));
        let mut all_times = Vec::new();
        let mut all_loops = 0u32;
        let mut rows = Vec::new();

        for problem in problems {
            let (out, times, correct) = run::run(problem)?;
            let (mean, range, cv) = run::summarise(&times, problem.loops as u32);
            let row = all::Row {
                n: problem.n,
                out: format_solution(out, correct),
                loops: problem.loops,
                mean: mean.into(),
                range: range.into(),
                cv: cv.into(),
            };
            rows.push(row);
            all_times.extend(times);
            all_loops += problem.loops;
        }

        // overall summary statistics
        println!(
            "{}
Problem count: {}
",
            "All problems".green().bold(),
            rows.len()
        );
        print_summary(all_times, all_loops);
        println!();

        cli_table::print_stdout(rows.with_title())?;

        // todo: output benchmarks to readme
    }

    Ok(())
}

fn print_summary(times: Vec<Duration>, loops: u32) {
    let (mean, (min, max), cv) = run::summarise(&times, loops);

    println!(
        r#"Mean: {mean:.2?} / loop
Range: [{min:.2?}, {max:.2?}]
CV: {cv:.2?}
Ran for: {loops} loops"#,
    );
}

/// Add commas to solutions and colour them based on whether they are correct or not.
fn format_solution<T: std::fmt::Display>(value: T, correct: Option<bool>) -> String {
    let s = value.to_string();
    // add commas
    let s = s
        .parse::<i64>()
        .map(|n| {
            n.to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(",")
        })
        .unwrap_or(s);
    // colourize
    let s = match correct {
        Some(true) => s.green().to_string(),
        Some(false) => s.red().to_string(),
        None => s.yellow().to_string(),
    };
    s
}

fn problem_url(n: usize) -> String {
    format!("https://projecteuler.net/problem={}", n)
}

/// Make a URL clickable using ANSI codes
fn hyperlink(url: String, text: String) -> String {
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}
