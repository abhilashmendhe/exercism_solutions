package logs

// Application identifies the application emitting the given log.
func Application(log string) string {
    for _, c := range log {
		if c == 10071 {
			return "recommendation"
		} else if c == 9728 {
			return "weather"
		} else if c == 128269 {
			return "search"
		}
	}
	return "default"
}

// Replace replaces all occurrences of old with new, returning the modified log
// to the caller.
func Replace(log string, oldRune, newRune rune) string {
	nlog := ""
	for _, num := range log {

		if num == oldRune {
			nlog += string(newRune)
		} else {
			nlog += string(num)
		}
	}
	return nlog
}

// WithinLimit determines whether or not the number of characters in log is
// within the limit.
func WithinLimit(log string, limit int) bool {
    c := 0

	for i, j := range log {
		if i == int(j) {
		}
		c++
	}
	if c <= limit {
		return true
	}
	return false
}
