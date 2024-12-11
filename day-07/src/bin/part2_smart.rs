use day_07::part2_smart::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}