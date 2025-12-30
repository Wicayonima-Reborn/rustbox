use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Creating new RustBox project: {}", name);
    Ok(())
}