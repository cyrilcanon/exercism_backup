// Package weather contains functions to forecast weather.
package weather

// CurrentCondition is a string describing the local weather condition.
var CurrentCondition string

// CurrentLocation is a string describing the current location.
var CurrentLocation string

// Forecast returns a string that descibe the current forecast.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
