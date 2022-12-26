use std::{collections::HashMap, io::stdin};

fn parse_snafu_number(s: &str) -> i64 {
    let digits = HashMap::from([('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)]);
    let mut result = 0;
    let mut power = 1;
    for c in s.chars().rev() {
        result += digits[&c] * power;
        power *= 5;
    }

    return result;
}

fn to_snafu_number(n: i64) -> String {
    let mut result = String::new();
    let mut remaining = n;

    while remaining > 0 {
        let digit = remaining % 5;
        let snafu_digit = match digit {
            3 => {
                remaining += 5;
                '='
            }
            4 => {
                remaining += 5;
                '-'
            }
            0 | 1 | 2 => char::from_digit(digit as u32, 10).unwrap(),
            _ => todo!(),
        };

        result.insert(0, snafu_digit);

        remaining /= 5;
    }

    return result;
}

fn main() {
    let fuel_sum: i64 = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_snafu_number(l.as_str()))
        .sum();
    let result = to_snafu_number(fuel_sum);

    println!("{}", result);
}
