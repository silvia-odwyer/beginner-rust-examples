package main
import (
	"time"
	"fmt"
	"math/rand"
)

func main () {
	then := time.Now()
	a := gen_random_array()

	for i := 0; i < len(a) - 1; i++{

		for j := 0; j < len(a) - 1; j++ {
			current_num := a[j]
			next_num := a[j + 1]
			if current_num > next_num {
				temp := current_num
				a[j] = next_num
				a[j + 1] = temp
			}
		}
	}
	fmt.Println(a)
	now := time.Now()
	diff := now.Sub(then)
	fmt.Println("FINISHED")
	fmt.Println("%f", diff)
}

func gen_random_array() []int {
	var a []int
	for i := 0; i < 100000; i++ {
		randomNumber := rand.Intn(1000000 - 1) + 1
		a = append(a, randomNumber)
	}
	return a
}  
