use std::{
    fs::{File, read_dir},
    io::{Result, Write},
    path::Path,
};

fn main() -> Result<()> {
    let src = Path::new("src");
    let mut f = File::create(src.join("lib.rs"))?;

    let mut problems = Vec::new();

    if let Ok(entries) = read_dir(src) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if name.starts_with('p') && name.len() > 1 {
                        if let Ok(_) = name[1..].parse::<u32>() {
                            problems.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    problems.sort_by_key(|p| p[1..].parse::<u32>().unwrap_or(0));

    writeln!(
        f,
        r#"//! AUTOGENERATED BY BUILD.RS
//! Please do not manually edit!
"#
    )?;
    for problem in &problems {
        writeln!(f, "pub mod {};", problem)?;
    }

    println!("cargo:rerun-if-changed=src");

    Ok(())
}
