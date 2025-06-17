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

pub struct DurationRange(Duration, Duration);

impl Cell for &DurationRange {
    fn cell(self) -> CellStruct {
        format!("[{:.2?}, {:.2?}]", self.0, self.1).cell()
    }
}

impl From<(Duration, Duration)> for DurationRange {
    fn from((a, b): (Duration, Duration)) -> Self {
        Self(a, b)
    }
}

#[derive(cli_table::Table)]
pub struct Row {
    #[table(title = "Problem", justify = "Justify::Center", bold)]
    pub n: usize,
    #[table(title = "Solution", justify = "Justify::Center")]
    pub out: String,
    #[table(title = "Loops", justify = "Justify::Center")]
    pub loops: u32,
    #[table(title = "Mean", justify = "Justify::Center")]
    pub mean: DurationCell,
    #[table(title = "CV", justify = "Justify::Center")]
    pub cv: DurationCell,
    #[table(title = "Range", justify = "Justify::Center")]
    pub range: DurationRange,
}
