use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.split("\n").into_iter();
    let seats: Vec<usize> = input.map(|input| parse_seat(input).2).collect();
    if cfg!(feature = "part-1") {
        println!("The answer is: {}", seats.clone().iter().max().unwrap());
    } else {
        let mut all_seats = vec![];
        // Add all seats apart from first and last row
        for row in 1..127 {
            for col in 0..=7 {
                all_seats.push((row * 8) + col);
            }
        }
        // Remove all known seats
        for seat in seats.clone().iter() {
            all_seats.retain(|val| *val != *seat);
        }
        // Check for -1 +1 seats
        let seat = all_seats
            .iter()
            .filter(|seat| {
                seats
                    .clone()
                    .iter()
                    .find(|prev| **prev == **seat - 1)
                    .is_some()
                    && seats
                        .clone()
                        .iter()
                        .find(|next| **next == **seat + 1)
                        .is_some()
            })
            .next()
            .unwrap();
        println!("The answer is: {}", seat);
    }
}

/// Returns row, column, seat id
fn parse_seat(input: &str) -> (usize, usize, usize) {
    let (rows, cols) = input.split_at(7).to_owned();
    let mut min = 0;
    let mut max = 127;
    for c in rows.chars() {
        if c == 'F' {
            max = min + (max - min) / 2;
        } else {
            min = min + ((max - min) / 2) + 1;
        }
    }
    let row = max;
    let mut min = 0;
    let mut max = 7;
    for c in cols.chars() {
        if c == 'L' {
            max = min + (max - min) / 2;
        } else {
            min = min + ((max - min) / 2) + 1;
        }
    }
    let col = max;
    (row, col, (row * 8) + col)
}

#[cfg(test)]
mod tests {
    use crate::parse_seat;

    #[test]
    fn part_1() {
        assert_eq!(parse_seat("FBFBBFFRLR"), (44, 5, 357));
        assert_eq!(parse_seat("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(parse_seat("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(parse_seat("BBFFBBFRLL"), (102, 4, 820));
    }
}
