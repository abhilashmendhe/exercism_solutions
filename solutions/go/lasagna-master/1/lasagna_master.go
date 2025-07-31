package lasagna

// TODO: define the 'PreparationTime()' function
func PreparationTime(layers []string, time int) int {
	if time < 1 {
		return len(layers) * 2
	}
	return len(layers) * time
}

// TODO: define the 'Quantities()' function
func Quantities(layers []string) (int, float64){
	noodles := 0
	sauce := 0.0
	for i := 0; i < len(layers); i++ {
		if layers[i] == "noodles" {
			noodles+=50
		}
		if layers[i] == "sauce" {
			sauce+=0.2
		}
	}
	return noodles, sauce
}
// TODO: define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendList []string, myList []string) []string {
	flen := len(friendList)
	mlen := len(myList)
	myList[mlen-1] = friendList[flen-1]
	return myList
}

// TODO: define the 'ScaleRecipe()' function
func ScaleRecipe(slices []float64, qunat int) []float64 {
	nslice := make([]float64, len(slices))

	for i := 0; i < len(slices); i++ {
		nslice[i] = slices[i]*float64(qunat) / 2
	}
	return nslice
}
// Your first steps could be to read through the tasks, and create
// these functions with their correct parameter lists and return types.
// The function body only needs to contain `panic("")`.
// 
// This will make the tests compile, but they will fail.
// You can then implement the function logic one by one and see
// an increasing number of tests passing as you implement more 
// functionality.
