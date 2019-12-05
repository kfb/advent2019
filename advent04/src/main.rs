use std::str::FromStr;

use regex::Regex;

fn digits_increase(num: &str) -> bool {
    let mut lowest_digit = 0;

    for c in num.chars() {
        let u32_value = u32::from_str(&c.to_string()).unwrap();
        if u32_value < lowest_digit {
            return false
        } else {
            lowest_digit = u32_value;
        }
    }

    true
}

fn correct_digits(num: &str, re: &Regex) -> bool {
    let lengths: Vec<usize> = re.find_iter(&num).map(
        |mat| mat.end() - mat.start()
    ).collect();

    lengths.contains(&(2 as usize))
}

fn main() {
    let mut count = 0;
    let re = Regex::new(r"0{2,}|1{2,}|2{2,}|3{2,}|4{2,}|5{2,}|6{2,}|7{2,}|8{2,}|9{2,}").unwrap();

    for num in 372037..905158 {
        let num_as_str = num.to_string();

        if !correct_digits(&num_as_str, &re) {
            continue
        }

        if digits_increase(&num_as_str) == false {
            continue
        }

        count += 1;
    }

    println!("Count: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_digits() {
        let re = Regex::new(r"0{2,}|1{2,}|2{2,}|3{2,}|4{2,}|5{2,}|6{2,}|7{2,}|8{2,}|9{2,}").unwrap();

        assert_eq!(correct_digits("112233", &re), true);
        assert_eq!(correct_digits("123444", &re), false);
        assert_eq!(correct_digits("111122", &re), true);
        assert_eq!(correct_digits("888999", &re), false);
        assert_eq!(correct_digits("899999", &re), false);
    }
}
