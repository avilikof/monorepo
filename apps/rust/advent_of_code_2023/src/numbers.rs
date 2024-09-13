pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}
impl Number {
    pub fn as_str(&self) -> &str {
        match self {
            Number::One => "one",
            Number::Two => "two",
            Number::Three => "three",
            Number::Four => "four",
            Number::Five => "five",
            Number::Six => "six",
            Number::Seven => "seven",
            Number::Eight => "eight",
            Number::Nine => "nine",
            Number::Zero => "zero",
        }
    }
    pub fn as_int(&self) -> u8 {
        match self {
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
            Number::Zero => 0,
        }
    }
    pub fn length(&self) -> usize {
        self.as_str().len()
    }
}
