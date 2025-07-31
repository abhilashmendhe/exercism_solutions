package chessboard

// Declare a type named File which stores if a square is occupied by a piece - this will be a slice of bools
type File [8]bool

// Declare a type named Chessboard which contains a map of eight Files, accessed with keys from "A" to "H"
type Chessboard map[string]File

// CountInFile returns how many squares are occupied in the chessboard,
// within the given file.
func CountInFile(cb Chessboard, file string) int {
	c := 0
	for i := 0; i < len(cb[file]); i++ {
		if cb[file][i] == true{
			c++
		}
	}
	return c
}

// CountInRank returns how many squares are occupied in the chessboard,
// within the given rank.
func CountInRank(cb Chessboard, rank int) int {
	if rank < 1 || rank > 8 {
		return 0
	}
	temp := []string{"A", "B", "C", "D", "E", "F", "G", "H"}
	c:=0
	for _, val := range temp {
		if cb[val][rank-1] == true{
			c++
		}
	}
	return c
}

// CountAll should count how many squares are present in the chessboard.
func CountAll(cb Chessboard) int {
	return 64
}

// CountOccupied returns how many squares are occupied in the chessboard.
func CountOccupied(cb Chessboard) int {
	c := 0
	temp := []string{"A", "B", "C", "D", "E", "F", "G", "H"}
	for _, val := range temp {
		for i := range []int{0, 1, 2, 3, 4, 5, 6, 7} {
			if cb[val][i] == true{
				c++
			}
		}
	}
	return c
}
