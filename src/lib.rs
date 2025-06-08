mod problems;
pub use problems::Problem;

pub mod prelude {
    pub use crate::problem;
    pub use color_eyre::{eyre::eyre as error, Result};
}

pub const PUBLIC_CHALLENGES: usize = 100;

pub mod regex {
    pub const LATEX: &'static str = r#"\$\$?([^$]+)\$?\$"#;
}
