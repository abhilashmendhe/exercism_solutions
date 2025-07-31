package parsinglogfiles

import (
    "regexp"
	"strings"
    )

func IsValidLine(text string) bool {
    re := regexp.MustCompile(`(\[ERR\]|\[DBG\]|\[INF\]|\[WRN]\|\[ERR\]|\[FTL\])`)
	v := re.FindStringIndex(text)
	if len(v) > 0{
		return v[0] == 0
	}
	return false
}

func SplitLogLine(text string) []string {
	re := regexp.MustCompile(`<[~*=-]*>`)
	v := re.Split(text, -1)
	return v
}

func CountQuotedPasswords(lines []string) int {
    re := regexp.MustCompile(`"[A-Za-z ]*pass(w|W)ord[A-Za-z ]*"`)
	c := 0
	for _, v := range lines {
		if re.MatchString(v) {
			c++
		}
	}
	return c
}

func RemoveEndOfLineText(text string) string {
    re := regexp.MustCompile(`end-of-line\d*`)
	return re.ReplaceAllString(text, "")
}

func TagWithUserName(lines []string) []string {
 	re := regexp.MustCompile(`User\s+[A-Za-z0-9]*`)
	for i := 0; i < len(lines); i++ {
		mst := re.FindString(lines[i])
		nmst := strings.Split(mst, " ")
        if len(mst) > 0 {
			lines[i] = "[USR] " + nmst[len(nmst)-1] + " " + lines[i]
		}
	}
	return lines
}
