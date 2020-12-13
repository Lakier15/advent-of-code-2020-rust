use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut trees: Vec<u64> = vec![];
    // (right, down)
    let slopes = if cfg!(feature = "part-1") {
        vec![(3, 1)]
    } else {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    };
    for slope in slopes {
        let (right, down) = slope;
        let mut col = 0;
        trees.push(0);
        let file = File::open("input.txt")?;
        let reader = BufReader::new(file);
        for (_, line) in reader
            .lines()
            .enumerate()
            .filter(|(row, _)| row % down == 0)
        {
            let val = &line?;
            if val.chars().nth(col).unwrap() == '#' {
                *trees.last_mut().unwrap() += 1;
            }
            col += right;
            col %= val.len();
        }
    }

    println!("The answer is: {}", trees.iter().fold(1, |acc, x| acc * *x));

    Ok(())
}
