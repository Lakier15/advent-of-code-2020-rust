use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::many0;
use nom::IResult;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut valid = 0;
    for line in reader.lines() {
        let val = parse(&line?).unwrap().1;

        if cfg!(feature = "part-1") {
            let count = val.pswd.chars().filter(|c| *c == val.cond).count();
            if (val.min..=val.max).contains(&count) {
                valid += 1;
            }
        } else {
            let first =
                val.min.checked_sub(1).and_then(|n| val.pswd.chars().nth(n)) == Some(val.cond);
            let second =
                val.max.checked_sub(1).and_then(|n| val.pswd.chars().nth(n)) == Some(val.cond);
            if first ^ second {
                valid += 1;
            }
        }
    }

    println!("The answer is: {}", valid);

    Ok(())
}

fn parse(input: &str) -> IResult<&str, Pswd> {
    let (input, min) = complete::digit1(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = complete::digit1(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, cond) = complete::alpha1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = complete::space1(input)?;
    let (input, pswd) = many0(complete::alpha1)(input)?;
    Ok((
        input,
        Pswd {
            min: usize::from_str(min).unwrap(),
            max: usize::from_str(max).unwrap(),
            cond: cond.chars().next().unwrap(),
            pswd: String::from(pswd[0]),
        },
    ))
}

#[derive(Debug)]
struct Pswd {
    min: usize,
    max: usize,
    cond: char,
    pswd: String,
}
