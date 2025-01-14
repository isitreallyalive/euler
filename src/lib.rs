use color_eyre::Result;

pub trait Execute {
    fn execute(&self) -> Result<()>;
}

pub const PUBLIC_CHALLENGES: usize = 100;

pub mod prelude {
    pub use super::Execute;
    pub use color_eyre::Result;
}

pub mod regex {
    pub const LATEX: &'static str = r#"\$\$?([^$]+)\$?\$"#;
}
