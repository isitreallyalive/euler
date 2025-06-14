use clap::Parser;
use cli_table::WithTitle;
use color_eyre::Result;
use euler::Problem;
use owo_colors::OwoColorize;
use std::{fs::OpenOptions, io::Write};

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
                let (out, times) = run::run(problem)?;
                let (total, mean, sd) = run::summarise(&times, problem.loops as u32);

                println!(
                    r#"{}
Solution: {}

Total time: {total:.2?}
Mean time: {mean:.2?} / loop
Std dev: {sd:.2?}
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
        let mut problems = Problem::all();
        problems.sort_by(|a, b| a.n.cmp(&b.n));
        let mut all_times = Vec::new();
        let mut all_loops = 0;
        let mut rows = Vec::new();

        for problem in problems {
            let (out, times) = run::run(problem)?;
            let (total, mean, sd) = run::summarise(&times, problem.loops as u32);
            let row = all::Row {
                n: problem.n,
                out,
                total: total.into(),
                mean: mean.into(),
                sd: sd.into(),
            };
            rows.push(row);
            all_times.extend(times);
            all_loops += problem.loops;
        }

        // overall summary statistics
        let (all_total, all_mean, all_sd) = run::summarise(&all_times, all_loops as u32);

        println!(
            r#"{}
Problem count: {}

Total time: {all_total:.2?}
Mean time: {all_mean:.2?} / loop
Std dev: {all_sd:.2?}
Ran for: {all_loops} loops
"#,
            "All problems".green().bold(),
            rows.len()
        );

        cli_table::print_stdout(rows.with_title())?;

        // todo: output benchmarks to readme
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
