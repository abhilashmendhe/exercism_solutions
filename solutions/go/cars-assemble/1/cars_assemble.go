package cars

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour.
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	return (float64(productionRate) * successRate / 100.0)
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute.
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	// return (productionRate * int(successRate) / 100.0) / 60
    return (int((float64(productionRate) * successRate / 100) / 60))
}

// CalculateCost works out the cost of producing the given number of cars.
func CalculateCost(carsCount int) uint {
	var ften int = carsCount / 10
	var sten int = carsCount % 10

	return uint(ften*95000) + uint(sten*10000)
}