package sorting

import (
    "reflect"
    "strconv"
	"fmt"
)
// DescribeNumber should return a string describing the number.
func DescribeNumber(f float64) string {
	return "This is the number " + strconv.FormatFloat(f, 'f', 1, 32)
}

type NumberBox interface {
	Number() int
}

// DescribeNumberBox should return a string describing the NumberBox.
func DescribeNumberBox(nb NumberBox) string {
	return "This is a box containing the number " + fmt.Sprintf("%.1f", float32(nb.Number()))
}

type FancyNumber struct {
	n string
}

func (i FancyNumber) Value() string {
	return i.n
}

type FancyNumberBox interface {
	Value() string
}

// ExtractFancyNumber should return the integer value for a FancyNumber
// and 0 if any other FancyNumberBox is supplied.
func ExtractFancyNumber(fnb FancyNumberBox) int {
    if reflect.TypeOf(fnb) != reflect.TypeOf(FancyNumber{}) {
		return 0
	}
	iv, err := strconv.Atoi(fnb.Value())
	fmt.Println(iv)
	if err != nil {
		return 0
	}
	return iv
}

// DescribeFancyNumberBox should return a string describing the FancyNumberBox.
func DescribeFancyNumberBox(fnb FancyNumberBox) string {
	msg := "This is a fancy box containing the number "
	v := ExtractFancyNumber(fnb)
	return msg + fmt.Sprintf("%.1f", float32(v))
}

// DescribeAnything should return a string describing whatever it contains.
func DescribeAnything(i interface{}) string {
	switch v := i.(type) {
	case float64:
		return DescribeNumber(v)
	case int:
		return DescribeNumber(float64(v))
    case NumberBox:
		return DescribeNumberBox(v)
	case FancyNumberBox:
		return DescribeFancyNumberBox(v)
	default:
		return "Return to sender"
	}
}
