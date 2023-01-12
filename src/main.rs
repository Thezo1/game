use crate::app::*;

mod logger;
mod test;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    Platform::new()
        .run()?;
    Ok(())
}
