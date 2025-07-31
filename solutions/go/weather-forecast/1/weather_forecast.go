// Package weather : this is the package where the weather_forecast.go.
package weather

// CurrentCondition : current conditions is string.
var CurrentCondition string

// CurrentLocation : current locations is also string.
var CurrentLocation string

// Forecast : function Forecast returns string, (current location plus current condition).
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
