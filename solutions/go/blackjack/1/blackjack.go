package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {

	switch card {
	case "ace":
		return 11
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	case "ten":
		return 10
	case "jack":
		return 10
	case "queen":
		return 10
	case "king":
		return 10
	default:
		return 0
	}
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	c1 := ParseCard(card1)
	c2 := ParseCard(card2)
	dc := ParseCard(dealerCard)

	var s int = c1 + c2

	if s <= 11 {
		return "H"
	} else if s >= 12 && s <= 16 {
		if dc >= 7 {
			return "H"
		} else {
			return "S"
		}
	} else if s >= 17 && s <= 20 {
		return "S"
	} else if s == 22 {
		return "P"
	} else {
		if s == 21 && dc < 10 {
			return "W"
		} else {
			return "S"
		}
	}
}