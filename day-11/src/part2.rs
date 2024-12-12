use crate::common::process_usize;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let result = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .fold(0, |acc, n| acc + process_usize(n, 75));
    Ok(result.to_string())
}
// 221632504974231

#[allow(unused)]
fn gen_lut() {
    print!("[");
    for n in 0..=999 {
        print!("[");

        for i in 0..=75 {
            let r = process_usize(n, i);
            print!("{r}");
            if i != 75 {
                print!(", ");
            }
        }
        println!("],");
    }
    println!("];");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("65601038650482", process(input)?);
        Ok(())
    }
}
