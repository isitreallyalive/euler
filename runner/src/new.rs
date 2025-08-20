use super::problem_url;
use color_eyre::Result;
use owo_colors::OwoColorize;
use regex::Regex;
use reqwest::blocking;
use scraper::{Html, Selector};
use std::{
    fs::{self, File},
    io::Write,
};

const LATEX: &str = r#"\$\$?([^$]+)\$?\$"#;
const REPLACEMENTS: phf::Map<&str, &str> = phf::phf_map! {
    "\\dots" => "...",
    "\\times" => "×",
    "\\," => ",",
    "\\lt" => "<",
    "\\to" => "→",
    "^2" => "²"
};

/// Fetch a problem from Project Euler's website
///
/// Returns, in the following order:
/// - The title of the problem
/// - A clean description of the problem
/// - The raw HTML of the description of the problem
pub fn fetch(n: usize) -> Option<(String, String, String)> {
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

    let description_el = {
        let selector = Selector::parse(".problem_content").ok()?;
        document.select(&selector).next()?
    };
    let description: String = {
        let regex = Regex::new(LATEX).ok()?;
        let mut text = regex
            .replace_all(&description_el.text().collect::<String>(), "$1")
            .to_string();

        for (from, to) in REPLACEMENTS.into_iter() {
            text = text.replace(from, to);
        }

        text
    };

    Some((
        title.trim().to_string(),
        description.trim().to_string(),
        description_el.html(),
    ))
}

/// Write the problem file.
pub fn problem(n: usize, title: &str, description: &str) -> Result<()> {
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
        .map(|l| format!("//* {l}\n"))
        .collect::<String>();

    write!(
        file,
        r#"//* Problem {n}: {title}
{description}
//! time complexity: O(?)
use euler::prelude::*;

fn solve() -> Solution {{
    unimplemented!();
}}

problem!({n}, solve);
"#
    )?;

    println!("{}", format!("Generated file for '{title}'").bold());

    Ok(())
}

/// Write a chapter in the book.
pub fn chapter(n: usize, title: &str, description: &str) -> Result<()> {
    // create the file
    let mut file = File::create(
        std::env::current_dir()?
            .join("book")
            .join("src")
            .join("problems")
            .join(format!("{n}.md")),
    )?;

    let mut description = description.to_owned();

    // handle individual rules like "n → n/2" and "n → 3n + 1"
    let rule_regex =
        Regex::new(r"^([a-zA-Z0-9+\-*/\s]+)\s*→\s*([a-zA-Z0-9+\-*/\s]+)\s*\(([^)]+)\)$")?;
    description = rule_regex
        .replace_all(&description, "$$$1 \\to $2$$ ($3)")
        .to_string();

    // handle sequences like "13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1"
    let sequence_regex = Regex::new(r"(\d+(?:\s*→\s*\d+)+\.?)")?;
    description = sequence_regex
        .replace_all(&description, "$$$$1$$")
        .to_string();

    // clean up any double dollar signs that might have been created
    description = description.replace("$$$$", "$$");

    // write the template
    write!(
        file,
        r#"# Problem {n}: {title}

## Problem

- [Link to the problem](https://projecteuler.net/problem={n})

{description}

## Solution

See it on [GitHub](https://github.com/isitreallyalive/euler/blob/main/problems/src/p{n}.rs).

```rs
{{{{#include ../../../problems/src/p{n}.rs}}}}
```"#
    )?;

    update_summary(n, title)?;
    println!("{}", format!("Generated chapter for '{title}'").bold());

    Ok(())
}

/// Update SUMMARY.md to include the new chapter
fn update_summary(n: usize, title: &str) -> Result<()> {
    let summary_path = std::env::current_dir()?
        .join("book")
        .join("src")
        .join("SUMMARY.md");
    let content = fs::read_to_string(&summary_path)?;
    let new_entry = format!("- [{title}](./problems/{n}.md)");

    // split the content to find where to insert the new entry
    let mut lines: Vec<&str> = content.lines().collect();
    let mut insert_index = None;
    let mut in_problems_section = false;
    let mut problem_start_index = None;

    for (i, line) in lines.iter().enumerate() {
        if line.contains("<!-- problems -->") {
            if !in_problems_section {
                in_problems_section = true;
                problem_start_index = Some(i + 1);
                continue;
            } else {
                // this is the closing comment
                insert_index = Some(i);
                break;
            }
        }
    }

    if let (Some(start), Some(end)) = (problem_start_index, insert_index) {
        // extract existing problem entries and parse their numbers
        let mut problem_entries: Vec<(usize, String)> = Vec::new();
        let re = Regex::new(r"\]\(./problems/(\d+)\.md\)")?;
        for line in lines.iter().take(end).skip(start) {
            if line.trim().starts_with("- [") || line.trim().starts_with('[') {
                // extract problem number from the file path
                if let Some(captures) = re.captures(line) {
                    if let Ok(problem_num) = captures[1].parse::<usize>() {
                        problem_entries.push((problem_num, line.to_string()));
                    }
                }
            }
        }

        // add the new entry
        problem_entries.push((n, new_entry));

        // sort by problem number
        problem_entries.sort_by_key(|(num, _)| *num);

        // remove old problem entries from lines
        for _ in start..end {
            if start < lines.len()
                && (lines[start].trim().starts_with("- [") || lines[start].trim().starts_with('['))
            {
                lines.remove(start);
            }
        }

        // insert sorted entries
        for (i, (_, entry)) in problem_entries.iter().enumerate() {
            lines.insert(start + i, entry);
        }

        // write
        let new_content = lines.join("\n");
        fs::write(&summary_path, new_content)?;
    }

    Ok(())
}
