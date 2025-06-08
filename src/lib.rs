mod problems;
pub use problems::Problem;

pub mod prelude {
    pub use crate::{error, problem};
    pub use color_eyre::Result;
}

pub const PUBLIC_CHALLENGES: usize = 100;

pub mod regex {
    pub const LATEX: &'static str = r#"\$\$?([^$]+)\$?\$"#;
}
