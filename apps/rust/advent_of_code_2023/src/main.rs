mod numbers;

use crate::numbers::Number;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let file_content = read_file("data/input_day_2txt");
    let buff_reader: BufReader<File>;
    match file_content {
        Ok(data) => {
            buff_reader = BufReader::new(data);
            loop_thru_line(buff_reader);
        }
        Err(err) => {
            println!("Error opening file: {}", err);
        }
    }
}

fn read_file(path: &str) -> Result<File, Error> {
    File::open(path)
}

fn loop_thru_line(buff_reader: BufReader<File>) {
    let mut calibration_values: Vec<u16> = Vec::new();
    for line in buff_reader.lines() {
        match line {
            Ok(line) => {
                // let numbers = extract_integers(&line);
                // let first_last_number = combine_first_last(&numbers);
                // calibration_values.push(first_last_number);
                let numbers = with_letters(&line);
                let first_last_number = combine_first_last(&numbers);
                calibration_values.push(first_last_number);
            }
            Err(err) => println!("{}", err),
        }
    }
    println!("Coordinates: {}", sum_all_ints(&calibration_values));
}

fn extract_integers(line: &str) -> Vec<u16> {
    let mut all_numbers: Vec<u16> = Vec::with_capacity(50);
    for char in line.chars() {
        if char.is_numeric() {
            let as_int = char.to_digit(10).unwrap() as u16;
            all_numbers.push(as_int);
        }
    }
    all_numbers
}

fn combine_first_last(numbers: &[u16]) -> u16 {
    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();
    first * 10 + last
}

fn sum_all_ints(numbers: &[u16]) -> u16 {
    let mut sum: u16 = 0;
    for num in numbers {
        sum += num;
    }
    sum
}

fn with_letters(line: &str) -> Vec<u16> {
    let mut numbers: Vec<u16> = Vec::new();
    let mut position: u8 = 0;
    let mut possible_number: Option<u8>;
    let mut index: u8 = 0;
    for letter in line.chars() {
        if index < position {
            index += 1;
            continue;
        }
        if letter.is_numeric() {
            numbers.push(letter.to_digit(10).unwrap() as u16);
        } else {
            match letter {
                'o' => (possible_number, position) = one(line, index),
                't' => (possible_number, position) = two_three(line, index),
                'f' => (possible_number, position) = four_five(line, index),
                's' => (possible_number, position) = six_seven(line, index),
                'e' => (possible_number, position) = eight(line, index),
                'n' => (possible_number, position) = nine(line, index),
                'z' => (possible_number, position) = zero(line, index),
                _ => possible_number = None,
            }
            match possible_number {
                None => {}
                Some(a) => {
                    numbers.push(a as u16);
                }
            }
        }
        index += 1;
    }
    numbers
}

fn compare_substring_to_word(
    line: &str,
    start: usize,
    end: usize,
    num: Number,
) -> (Option<u8>, u8) {
    match get_substring(line, start, end) {
        None => (None, start as u8),
        Some(sub_string) => {
            if sub_string == num.as_str() {
                (Some(num.as_int()), end as u8)
            } else {
                (None, start as u8)
            }
        }
    }
}

fn one(line: &str, index: u8) -> (Option<u8>, u8) {
    compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::One.length(),
        Number::One,
    )
}
fn two_three(line: &str, index: u8) -> (Option<u8>, u8) {
    let (number, position) = compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::Two.length(),
        Number::Two,
    );
    match number {
        None => compare_substring_to_word(
            line,
            index as usize,
            index as usize + Number::Three.length(),
            Number::Three,
        ),
        Some(_) => (number, position),
    }
}
fn four_five(line: &str, index: u8) -> (Option<u8>, u8) {
    let (number, position) = compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::Four.length(),
        Number::Four,
    );
    match number {
        None => compare_substring_to_word(
            line,
            index as usize,
            index as usize + Number::Five.length(),
            Number::Five,
        ),
        Some(_) => (number, position),
    }
}
fn six_seven(line: &str, index: u8) -> (Option<u8>, u8) {
    let (number, position) = compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::Six.length(),
        Number::Six,
    );
    match number {
        None => compare_substring_to_word(
            line,
            index as usize,
            index as usize + Number::Seven.length(),
            Number::Seven,
        ),
        Some(_) => (number, position),
    }
}
fn eight(line: &str, index: u8) -> (Option<u8>, u8) {
    compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::Eight.length(),
        Number::Eight,
    )
}
fn nine(line: &str, index: u8) -> (Option<u8>, u8) {
    compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::Nine.length(),
        Number::Nine,
    )
}
fn zero(line: &str, index: u8) -> (Option<u8>, u8) {
    compare_substring_to_word(
        line,
        index as usize,
        index as usize + Number::Zero.length(),
        Number::Zero,
    )
}

fn get_substring(s: &str, start: usize, end: usize) -> Option<String> {
    let char_indices: Vec<_> = s.char_indices().collect();
    if start > end || end > s.len() {
        println!("issues with substring verification:\n{s}\n{start}\n{end}");
        return None;
    }
    let start_byte = char_indices.get(start)?.0;
    let end_byte = char_indices.get(end - 1)?.0;
    let sub_string = s[start_byte..end_byte + 1].to_string();
    Some(sub_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let line = String::from("zero");
        let index = 0; // The starting index of the substring "zero"
        let (possible_number, new_position) = zero(&line, index);

        assert_eq!(possible_number, Some(0));
        assert_eq!(new_position, index + Number::Zero.length() as u8);
    }
    #[test]
    fn test_zero_enum() {
        assert_eq!(Number::Zero.as_str(), "zero");
        assert_eq!(Number::Zero.length(), 4);
    }
    #[test]
    fn test_number_at_the_end() {
        let line = String::from("This is 01234zero");
        let index = 13; // The starting index of the substring "zero"
        let (possible_number, new_position) = zero(&line, index);

        assert_eq!(possible_number, Some(0));
        assert_eq!(new_position, index + Number::Zero.length() as u8);
    }
    #[test]
    fn test_number_at_the_beginning() {
        let line = String::from("zero This is 01234");
        let index = 0; // The starting index of the substring "zero"
        let (possible_number, new_position) = zero(&line, index);

        assert_eq!(possible_number, Some(0));
        assert_eq!(new_position, index + Number::Zero.length() as u8);
    }
    #[test]
    fn test_zero_in_the_middle() {
        let line = String::from("Thisiszero01234");
        let index = 6; // The starting index of the substring "zero"
        let (possible_number, new_position) = zero(&line, index);

        assert_eq!(possible_number, Some(0));
        assert_eq!(new_position, index + Number::Zero.length() as u8);
    }
    #[test]
    fn test_one() {
        let line = String::from("one");
        let index = 0; // The starting index of the substring "one"
        let (possible_number, new_position) = one(&line, index);

        assert_eq!(possible_number, Some(1));
        assert_eq!(new_position, index + Number::One.length() as u8);
    }

    #[test]
    fn test_one_enum() {
        assert_eq!(Number::One.as_str(), "one");
        assert_eq!(Number::One.length(), 3);
    }

    #[test]
    fn test_one_at_the_end() {
        let line = String::from("This is 01234one");
        let index = 13; // The starting index of the substring "one"
        let (possible_number, new_position) = one(&line, index);

        assert_eq!(possible_number, Some(1));
        assert_eq!(new_position, index + Number::One.length() as u8);
    }

    #[test]
    fn test_one_at_the_beginning() {
        let line = String::from("one This is 01234");
        let index = 0; // The starting index of the substring "one"
        let (possible_number, new_position) = one(&line, index);

        assert_eq!(possible_number, Some(1));
        assert_eq!(new_position, index + Number::One.length() as u8);
    }

    #[test]
    fn test_one_in_the_middle() {
        let line = String::from("Thisisone01234");
        let index = 6; // The starting index of the substring "one"
        let (possible_number, new_position) = one(&line, index);

        assert_eq!(possible_number, Some(1));
        assert_eq!(new_position, index + Number::One.length() as u8);
    }
    #[test]
    fn test_two() {
        let line = String::from("two");
        let index = 0; // The starting index of the substring "two"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(2));
        assert_eq!(new_position, index + Number::Two.length() as u8);
    }

    #[test]
    fn test_two_enum() {
        assert_eq!(Number::Two.as_str(), "two");
        assert_eq!(Number::Two.length(), 3);
    }

    #[test]
    fn test_two_at_the_end() {
        let line = String::from("This is 01234two");
        let index = 13; // The starting index of the substring "two"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(2));
        assert_eq!(new_position, index + Number::Two.length() as u8);
    }

    #[test]
    fn test_two_at_the_beginning() {
        let line = String::from("two This is 01234");
        let index = 0; // The starting index of the substring "two"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(2));
        assert_eq!(new_position, index + Number::Two.length() as u8);
    }

    #[test]
    fn test_two_in_the_middle() {
        let line = String::from("Thisistwo01234");
        let index = 6; // The starting index of the substring "two"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(2));
        assert_eq!(new_position, index + Number::Two.length() as u8);
    }

    #[test]
    fn test_three() {
        let line = String::from("three");
        let index = 0; // The starting index of the substring "three"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(3));
        assert_eq!(new_position, index + Number::Three.length() as u8);
    }

    #[test]
    fn test_three_enum() {
        assert_eq!(Number::Three.as_str(), "three");
        assert_eq!(Number::Three.length(), 5);
    }

    #[test]
    fn test_three_at_the_end() {
        let line = String::from("This is 01234three");
        let index = 13; // The starting index of the substring "three"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(3));
        assert_eq!(new_position, index + Number::Three.length() as u8);
    }

    #[test]
    fn test_three_at_the_beginning() {
        let line = String::from("three This is 01234");
        let index = 0; // The starting index of the substring "three"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(3));
        assert_eq!(new_position, index + Number::Three.length() as u8);
    }

    #[test]
    fn test_three_in_the_middle() {
        let line = String::from("Thisisthree01234");
        let index = 6; // The starting index of the substring "three"
        let (possible_number, new_position) = two_three(&line, index);

        assert_eq!(possible_number, Some(3));
        assert_eq!(new_position, index + Number::Three.length() as u8);
    }

    #[test]
    fn test_four() {
        let line = String::from("four");
        let index = 0; // The starting index of the substring "four"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(4));
        assert_eq!(new_position, index + Number::Four.length() as u8);
    }

    #[test]
    fn test_four_enum() {
        assert_eq!(Number::Four.as_str(), "four");
        assert_eq!(Number::Four.length(), 4);
    }

    #[test]
    fn test_four_at_the_end() {
        let line = String::from("This is 01234four");
        let index = 13; // The starting index of the substring "four"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(4));
        assert_eq!(new_position, index + Number::Four.length() as u8);
    }

    #[test]
    fn test_four_at_the_beginning() {
        let line = String::from("four This is 01234");
        let index = 0; // The starting index of the substring "four"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(4));
        assert_eq!(new_position, index + Number::Four.length() as u8);
    }

    #[test]
    fn test_four_in_the_middle() {
        let line = String::from("Thisisfour01234");
        let index = 6; // The starting index of the substring "four"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(4));
        assert_eq!(new_position, index + Number::Four.length() as u8);
    }

    #[test]
    fn test_five() {
        let line = String::from("five");
        let index = 0; // The starting index of the substring "five"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(5));
        assert_eq!(new_position, index + Number::Five.length() as u8);
    }

    #[test]
    fn test_five_enum() {
        assert_eq!(Number::Five.as_str(), "five");
        assert_eq!(Number::Five.length(), 4);
    }

    #[test]
    fn test_five_at_the_end() {
        let line = String::from("This is 01234five");
        let index = 13; // The starting index of the substring "five"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(5));
        assert_eq!(new_position, index + Number::Five.length() as u8);
    }

    #[test]
    fn test_five_at_the_beginning() {
        let line = String::from("five This is 01234");
        let index = 0; // The starting index of the substring "five"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(5));
        assert_eq!(new_position, index + Number::Five.length() as u8);
    }

    #[test]
    fn test_five_in_the_middle() {
        let line = String::from("Thisisfive01234");
        let index = 6; // The starting index of the substring "five"
        let (possible_number, new_position) = four_five(&line, index);

        assert_eq!(possible_number, Some(5));
        assert_eq!(new_position, index + Number::Five.length() as u8);
    }
    #[test]
    fn test_six() {
        let line = String::from("six");
        let index = 0; // The starting index of the substring "six"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(6));
        assert_eq!(new_position, index + Number::Six.length() as u8);
    }

    #[test]
    fn test_six_enum() {
        assert_eq!(Number::Six.as_str(), "six");
        assert_eq!(Number::Six.length(), 3);
    }

    #[test]
    fn test_six_at_the_end() {
        let line = String::from("This is 01234six");
        let index = 13; // The starting index of the substring "six"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(6));
        assert_eq!(new_position, index + Number::Six.length() as u8);
    }

    #[test]
    fn test_six_at_the_beginning() {
        let line = String::from("six This is 01234");
        let index = 0; // The starting index of the substring "six"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(6));
        assert_eq!(new_position, index + Number::Six.length() as u8);
    }

    #[test]
    fn test_six_in_the_middle() {
        let line = String::from("Thisissix01234");
        let index = 6; // The starting index of the substring "six"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(6));
        assert_eq!(new_position, index + Number::Six.length() as u8);
    }

    // Tests for the number seven
    #[test]
    fn test_seven() {
        let line = String::from("seven");
        let index = 0; // The starting index of the substring "seven"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(7));
        assert_eq!(new_position, index + Number::Seven.length() as u8);
    }

    #[test]
    fn test_seven_enum() {
        assert_eq!(Number::Seven.as_str(), "seven");
        assert_eq!(Number::Seven.length(), 5);
    }

    #[test]
    fn test_seven_at_the_end() {
        let line = String::from("This is 01234seven");
        let index = 13; // The starting index of the substring "seven"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(7));
        assert_eq!(new_position, index + Number::Seven.length() as u8);
    }

    #[test]
    fn test_seven_at_the_beginning() {
        let line = String::from("seven This is 01234");
        let index = 0; // The starting index of the substring "seven"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(7));
        assert_eq!(new_position, index + Number::Seven.length() as u8);
    }

    #[test]
    fn test_seven_in_the_middle() {
        let line = String::from("Thisisseven01234");
        let index = 6; // The starting index of the substring "seven"
        let (possible_number, new_position) = six_seven(&line, index);

        assert_eq!(possible_number, Some(7));
        assert_eq!(new_position, index + Number::Seven.length() as u8);
    }

    // Tests for the number eight
    #[test]
    fn test_eight() {
        let line = String::from("eight");
        let index = 0; // The starting index of the substring "eight"
        let (possible_number, new_position) = eight(&line, index);

        assert_eq!(possible_number, Some(8));
        assert_eq!(new_position, index + Number::Eight.length() as u8);
    }

    #[test]
    fn test_eight_enum() {
        assert_eq!(Number::Eight.as_str(), "eight");
        assert_eq!(Number::Eight.length(), 5);
    }

    #[test]
    fn test_eight_at_the_end() {
        let line = String::from("This is 01234eight");
        let index = 13; // The starting index of the substring "eight"
        let (possible_number, new_position) = eight(&line, index);

        assert_eq!(possible_number, Some(8));
        assert_eq!(new_position, index + Number::Eight.length() as u8);
    }

    #[test]
    fn test_eight_at_the_beginning() {
        let line = String::from("eight This is 01234");
        let index = 0; // The starting index of the substring "eight"
        let (possible_number, new_position) = eight(&line, index);

        assert_eq!(possible_number, Some(8));
        assert_eq!(new_position, index + Number::Eight.length() as u8);
    }

    #[test]
    fn test_eight_in_the_middle() {
        let line = String::from("Thisiseight01234");
        let index = 6; // The starting index of the substring "eight"
        let (possible_number, new_position) = eight(&line, index);

        assert_eq!(possible_number, Some(8));
        assert_eq!(new_position, index + Number::Eight.length() as u8);
    }

    // Tests for the number nine
    #[test]
    fn test_nine() {
        let line = String::from("nine");
        let index = 0; // The starting index of the substring "nine"
        let (possible_number, new_position) = nine(&line, index);

        assert_eq!(possible_number, Some(9));
        assert_eq!(new_position, index + Number::Nine.length() as u8);
    }

    #[test]
    fn test_nine_enum() {
        assert_eq!(Number::Nine.as_str(), "nine");
        assert_eq!(Number::Nine.length(), 4);
    }

    #[test]
    fn test_nine_at_the_end() {
        let line = String::from("This is 01234nine");
        let index = 13; // The starting index of the substring "nine"
        let (possible_number, new_position) = nine(&line, index);

        assert_eq!(possible_number, Some(9));
        assert_eq!(new_position, index + Number::Nine.length() as u8);
    }

    #[test]
    fn test_nine_at_the_beginning() {
        let line = String::from("nine This is 01234");
        let index = 0; // The starting index of the substring "nine"
        let (possible_number, new_position) = nine(&line, index);

        assert_eq!(possible_number, Some(9));
        assert_eq!(new_position, index + Number::Nine.length() as u8);
    }

    #[test]
    fn test_nine_in_the_middle() {
        let line = String::from("Thisisnine01234");
        let index = 6; // The starting index of the substring "nine"
        let (possible_number, new_position) = nine(&line, index);

        assert_eq!(possible_number, Some(9));
        assert_eq!(new_position, index + Number::Nine.length() as u8);
    }
}
