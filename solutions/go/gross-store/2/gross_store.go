package gross

// Units stores the Gross Store unit measurements.
func Units() map[string]int {
	gunit := make(map[string]int)
	gunit["quarter_of_a_dozen"] = 3
	gunit["half_of_a_dozen"] = 6
	gunit["dozen"] = 12
	gunit["small_gross"] = 120
	gunit["gross"] = 144
	gunit["great_gross"] = 1728
	return gunit
}

// NewBill creates a new bill.
func NewBill() map[string]int {
	return map[string]int{}
}

// AddItem adds an item to customer bill.
func AddItem(bill, units map[string]int, item, unit string) bool {
	vu, vex := units[unit]
	_, bex := bill[item]
	
	if !vex {
		return false
	}

	if !bex {
		bill[item] = vu
	} else {
		bill[item] += vu
	}
	return true
}


// RemoveItem removes an item from customer bill.
func RemoveItem(bill, units map[string]int, item, unit string) bool {

    vu, vex := units[unit]
	bu, bex := bill[item]
	if !bex || !vex || vu>bu {
		return false
	} else if bu==vu {
		delete(bill, item)
	} else {
		bill[item] -= vu
	}
return true;
}

// GetItem returns the quantity of an item that the customer has in his/her bill.
func GetItem(bill map[string]int, item string) (int, bool) {
	v, exists := bill[item]
	if !exists {
		return 0, false
	}
	return v, true
}
