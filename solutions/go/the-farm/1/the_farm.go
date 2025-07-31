package thefarm

import (
    "errors"
    "strconv"
    )
// See types.go for the types defined for this exercise.

// TODO: Define the SillyNephewError type here.
type SillyNephewError struct {
	message string
}
// DivideFood computes the fodder amount per cow for the given cows.
func DivideFood(weightFodder WeightFodder, cows int) (float64, error) {

    famt, err := weightFodder.FodderAmount()
	if cows == 0{
        return 0.0, errors.New("division by zero")
    }
	if err != nil {
		if err.Error() == "sensor error" {
			if famt > 0 {
				famt *= 2
				return famt / float64(cows), nil
            } else {
            	n_err := errors.New("negative fodder")
				return 0.0, n_err
			}
		} else {
			return 0.0, err
		}

	}
	if famt < 0 {
		return 0.0, errors.New("negative fodder")
	}
	var nepherr = &SillyNephewError{message: "silly nephew, there cannot be " + strconv.Itoa(cows) + " cows"}
	if cows < 0 {
		return 0.0, errors.New(nepherr.message)
	}
	return famt / float64(cows), nil
}
