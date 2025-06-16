use cli_table::{Cell, CellStruct, format::Justify};
use std::time::Duration;

pub struct DurationCell(Duration);

impl Cell for &DurationCell {
    fn cell(self) -> CellStruct {
        format!("{:.2?}", self.0).cell()
    }
}

impl From<Duration> for DurationCell {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

#[derive(cli_table::Table)]
pub struct Row {
    #[table(title = "Problem", justify = "Justify::Center", bold)]
    pub n: usize,
    #[table(title = "Solution", justify = "Justify::Center")]
    pub out: String,
    #[table(title = "Loops", justify = "Justify::Center")]
    pub loops: usize,
    #[table(title = "Mean time", justify = "Justify::Center")]
    pub mean: DurationCell,
    #[table(title = "Std dev", justify = "Justify::Center")]
    pub sd: DurationCell,
}
