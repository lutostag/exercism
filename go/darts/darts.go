package darts

import "math"

func Score(x float64, y float64) int {
	dist := math.Sqrt(x*x + y*y)
	if dist <= 1.0 {
		return 10
	} else if dist <= 5.0 {
		return 5
	} else if dist <= 10.0 {
		return 1
	} else {
		return 0
	}

}
