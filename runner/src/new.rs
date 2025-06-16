use super::problem_url;
use color_eyre::Result;
use owo_colors::OwoColorize;
use regex::Regex;
use reqwest::blocking;
use scraper::{Html, Selector};
use std::{fs::File, io::Write};

const LATEX: &'static str = r#"\$\$?([^$]+)\$?\$"#;
const REPLACEMENTS: phf::Map<&str, &str> = phf::phf_map! {
    r#"\dots"# => "...",
    r#"\times"# => "×",
    r#"\,"# => ",",
    r#"\lt"# => "<",
    "^2" => "²"
};

/// Fetch a problem from Project Euler's website
pub fn fetch(n: usize) -> Option<(String, String)> {
    let res = blocking::get(problem_url(n)).ok()?;

    // check if it exists
    let exists = res.url().path() != "/archives";

    if !exists {
        return None;
    };

    // find the title and description
    let document = Html::parse_document(&res.text().ok()?);
    let title: String = {
        let selector = Selector::parse("h2").ok()?;
        document.select(&selector).next()?.text().collect()
    };
    let description: String = {
        let selector = Selector::parse(".problem_content").ok()?;
        let regex = Regex::new(LATEX).ok()?;
        let mut text = regex
            .replace_all(
                &document
                    .select(&selector)
                    .next()?
                    .text()
                    .collect::<String>(),
                "$1",
            )
            .to_string();

        for (from, to) in REPLACEMENTS.into_iter() {
            text = text.replace(from, to);
        }

        text
    };

    Some((title.trim().to_string(), description.trim().to_string()))
}

/// Write the problem file.
pub fn write(n: usize, title: String, description: String) -> Result<()> {
    // create the file
    let mut file = File::create(
        std::env::current_dir()?
            .join("problems")
            .join("src")
            .join(format!("p{n}.rs")),
    )?;

    // write the template
    let description = description
        .lines()
        .map(|l| format!("//! {l}\n"))
        .collect::<String>();

    write!(
        file,
        r#"//! Problem {n}: {title}
{description}
// todo: time complexity: O(?)
use euler::prelude::*;

fn solve() -> Solution {{
    unimplemented!();
}}

problem!({n}, solve);
"#
    )?;

    println!("{}", format!("Generated problems/src/p{n}.rs").bold());

    Ok(())
}
