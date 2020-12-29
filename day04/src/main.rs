use std::fs::read_to_string;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::verify;
use nom::error::ErrorKind;
use nom::IResult;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.split("\n\n").into_iter();
    let input = input.map(|entry| entry.replace('\n', " ")).collect();
    println!("The answer is: {}", parse(input));
}

fn parse(input: Vec<String>) -> usize {
    let present_fields = input
        .into_iter()
        .filter(|entry| are_fields_present(entry.as_str()));
    if cfg!(feature = "part-1") {
        present_fields.count()
    } else {
        present_fields
            .filter(|entry| are_fields_valid(entry.as_str()).is_some())
            .count()
    }
}

fn are_fields_present(string: &str) -> bool {
    string.contains("byr:")
        && string.contains("iyr:")
        && string.contains("eyr:")
        && string.contains("hgt:")
        && string.contains("hcl:")
        && string.contains("ecl:")
        && string.contains("pid:")
}

fn are_fields_valid(input: &str) -> Option<()> {
    let _ = parse_byr(input).ok().map(|_| ())?;
    let _ = parse_iyr(input).ok().map(|_| ())?;
    let _ = parse_eyr(input).ok().map(|_| ())?;
    let _ = parse_hgt(input).ok().map(|_| ())?;
    let _ = parse_hcl(input).ok().map(|_| ())?;
    let _ = parse_ecl(input)?;
    parse_pid(input).ok().map(|_| ())
}

fn parse_byr(input: &str) -> IResult<&str, &str> {
    if let Some(mut start) = input.find("byr:") {
        start += 4;
        verify(complete::digit1, |val| {
            if let Ok(num) = str::parse::<u16>(val) {
                (1920..=2002).contains(&num)
            } else {
                false
            }
        })(&input[start..])
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Verify,
        )))
    }
}

fn parse_iyr(input: &str) -> IResult<&str, &str> {
    if let Some(mut start) = input.find("iyr:") {
        start += 4;
        verify(complete::digit1, |val| {
            if let Ok(num) = str::parse::<u16>(val) {
                (2010..=2020).contains(&num)
            } else {
                false
            }
        })(&input[start..])
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Verify,
        )))
    }
}

fn parse_eyr(input: &str) -> IResult<&str, &str> {
    if let Some(mut start) = input.find("eyr:") {
        start += 4;
        verify(complete::digit1, |val| {
            if let Ok(num) = str::parse::<u16>(val) {
                (2020..=2030).contains(&num)
            } else {
                false
            }
        })(&input[start..])
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Verify,
        )))
    }
}

fn parse_hgt(input: &str) -> IResult<&str, &str> {
    let err = Err(nom::Err::Error(nom::error::Error::new(
        input,
        ErrorKind::Verify,
    )));
    if let Some(mut start) = input.find("hgt:") {
        start += 4;
        let (input, num) = complete::digit1(&input[start..])?;
        if let Ok(num) = str::parse::<u8>(num) {
            let res_cm = tag("cm")(input);
            let res_in = tag("in")(input);
            if res_cm.is_ok() && (150..=193).contains(&num) {
                res_cm
            } else if res_in.is_ok() && (59..=76).contains(&num) {
                res_in
            } else {
                err
            }
        } else {
            err
        }
    } else {
        err
    }
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    let error = Err(nom::Err::Error(nom::error::Error::new(
        input,
        ErrorKind::Verify,
    )));
    if let Some(mut start) = input.find("hcl:") {
        start += 4;
        let (input, _) = tag("#")(&input[start..])?;
        verify(complete::hex_digit1, |s: &str| s.len() == 6)(input)
    } else {
        error
    }
}

fn parse_ecl(input: &str) -> Option<()> {
    input.find("ecl:").and_then(|mut start| {
        start += 4;
        match &input[start..(start + 3)] {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(()),
            _ => None,
        }
    })
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    let err = Err(nom::Err::Error(nom::error::Error::new(
        input,
        ErrorKind::Verify,
    )));
    if let Some(mut start) = input.find("pid:") {
        start += 4;
        verify(complete::alphanumeric1, |s: &str| s.len() == 9)(&input[start..])
    } else {
        err
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "part-1")]
    #[test]
    fn count_valid() {
        let input = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        assert_eq!(parse(input), 2);
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn byr() {
        assert!(parse_byr("byr:1919").is_err());
        assert!(parse_byr("byr:1920").is_ok());
        assert!(parse_byr("byr:2002").is_ok());
        assert!(parse_byr("byr:2003").is_err());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn iyr() {
        assert!(parse_iyr("iyr:2009").is_err());
        assert!(parse_iyr("iyr:2010").is_ok());
        assert!(parse_iyr("iyr:2020").is_ok());
        assert!(parse_iyr("iyr:2021").is_err());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn eyr() {
        assert!(parse_eyr("eyr:2019").is_err());
        assert!(parse_eyr("eyr:2020").is_ok());
        assert!(parse_eyr("eyr:2030").is_ok());
        assert!(parse_eyr("eyr:2031").is_err());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn hgt() {
        assert!(parse_hgt("hgt:58in").is_err());
        assert!(parse_hgt("hgt:59in").is_ok());
        assert!(parse_hgt("hgt:60in").is_ok());
        assert!(parse_hgt("hgt:76in").is_ok());
        assert!(parse_hgt("hgt:77in").is_err());
        assert!(parse_hgt("hgt:149cm").is_err());
        assert!(parse_hgt("hgt:150cm").is_ok());
        assert!(parse_hgt("hgt:190cm").is_ok());
        assert!(parse_hgt("hgt:193cm").is_ok());
        assert!(parse_hgt("hgt:194cm").is_err());
        assert!(parse_hgt("hgt:190in").is_err());
        assert!(parse_hgt("hgt:190").is_err());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn hcl() {
        assert!(parse_hcl("hcl:#123abc").is_ok());
        assert!(parse_hcl("hcl:#123abcde").is_err());
        assert!(parse_hcl("hcl:#123abzza").is_err());
        assert!(parse_hcl("hcl:#123abz").is_err());
        assert!(parse_hcl("hcl:123abc").is_err());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn ecl() {
        assert!(parse_ecl("ecl:brn").is_some());
        assert!(parse_ecl("ecl:wat").is_none());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn pid() {
        assert!(parse_pid("pid:000000001").is_ok());
        assert!(parse_pid("pid:0123456789").is_err());
        assert!(parse_pid("pid:01234567").is_err());
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn count_valid_none() {
        let input = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007        
"#;
        let input = input.split("\n\n").into_iter();
        let input = input.map(|entry| entry.replace('\n', " ")).collect();
        assert_eq!(parse(input), 0);
    }

    #[cfg(not(feature = "part-1"))]
    #[test]
    fn count_valid_all() {
        let input = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719        
"#;
        let input = input.split("\n\n").into_iter();
        let input = input.map(|entry| entry.replace('\n', " ")).collect();
        assert_eq!(parse(input), 4);
    }
}
