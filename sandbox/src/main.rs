use anyhow::{Context, Result};

fn foo() -> Result<()> {
    bar().context("foo error")?;
    Ok(())
}

fn bar() -> Result<()> {
    baz().context("bar error")?;
    Ok(())
}

fn baz() -> Result<()> {
    Err(anyhow::anyhow!("baz error"))
}

fn main() {
    if let Err(e) = foo() {
        println!("{:?}", e);
    }
}
