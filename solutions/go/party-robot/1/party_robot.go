package partyrobot
import (
	"strconv"
)
// Welcome greets a person by name.
func Welcome(name string) string {
	return "Welcome to my party, " + name +"!"
}

// HappyBirthday wishes happy birthday to the birthday person and exclaims their age.
func HappyBirthday(name string, age int) string {
	return "Happy birthday " + name + "! You are now " + strconv.Itoa(age) + " years old!"
}

// AssignTable assigns a table to each guest.
func AssignTable(name string, table int, neighbor, direction string, distance float64) string {
	var stable string
	
	if table/10 == 0 {
		stable = "00" + strconv.Itoa(table)
	} else if table/100 == 0 {
		stable = "0" + strconv.Itoa(table)
	} else {
		stable = strconv.Itoa(table)
	}
	return "Welcome to my party, "+name+"!\nYou have been assigned to table "+stable+". Your table is "+direction+", exactly "+strconv.FormatFloat(distance,'f', 1, 32)+" meters from here.\nYou will be sitting next to "+neighbor+"."
}
