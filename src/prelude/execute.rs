use color_eyre::Result;

#[allow(non_camel_case_types)]
pub enum Return {
    None,
    u32(u32),
}

pub trait Execute {
    fn execute(&self) -> Result<Return>;
}
