use day_14::part2_optimized::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1mia.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}