package adventofcode2023

// Define the custom type
type Digit int

// Define the constants for each digit
const (
	One Digit = iota + 1
	Two
	Three
	Four
	Five
	Six
	Seven
	Eight
	Nine
)

// Implement the methods for the custom type
func (d Digit) AsString() string {
	switch d {
	case One:
		return "one"
	case Two:
		return "two"
	case Three:
		return "three"
	case Four:
		return "four"
	case Five:
		return "five"
	case Six:
		return "six"
	case Seven:
		return "seven"
	case Eight:
		return "eight"
	case Nine:
		return "nine"
	default:
		return "unknown"
	}
}

func (d Digit) AsInt() int {
	return int(d)
}

func (d Digit) Len() int {
	return len(d.AsString())
}
