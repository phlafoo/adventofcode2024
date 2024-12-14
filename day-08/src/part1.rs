#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let dim = input.find('\n').unwrap() + 1;
    let dim_i32 = dim as i32;

    let input = input.as_bytes();

    // Save positions of antinodes
    let mut nodes = vec![false; dim * (dim - 1)];

    // b'0'..b'z' => 60..172 => 113 possible antenna types
    let mut antennas = [const { Vec::new() }; 113];

    let mut result = 0;

    for (i, &antenna) in input
        .iter()
        .enumerate()
        .filter(|(_, &a)| a != b'.' && a != b'\n')
    {
        // Get antenna locations that match this frequency
        let ants = &mut antennas[(antenna - b'0') as usize];
        let (x, y) = ((i % dim) as i32, (i / dim) as i32);

        for (xo, yo) in ants.iter() {
            // Coord difference
            let (dx, dy) = (xo - x, yo - y);

            // Check first antinode
            let (nx, ny) = (xo + dx, yo + dy);
            if (0..dim_i32 - 1).contains(&nx) && (0..dim_i32 - 1).contains(&ny) {
                let n = (ny * dim_i32 + nx) as usize;
                if !nodes[n] {
                    nodes[n] = true;
                    result += 1;
                }
            }
            // Check second antinode
            let (nx, ny) = (x - dx, y - dy);
            if (0..dim_i32 - 1).contains(&nx) && (0..dim_i32 - 1).contains(&ny) {
                let n = (ny * dim_i32 + nx) as usize;
                if !nodes[n] {
                    nodes[n] = true;
                    result += 1;
                }
            }
        }
        ants.push((x, y));
    }

    Ok(result.to_string())
}
// 220

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
