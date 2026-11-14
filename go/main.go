// x ^ dlx
// dly = sx ^ s(x ^ dlx)


// package main
// import "fmt"

// func main()  {
// 	var x = 5
// 	var dlx = 3
// 	sbox := [8]int{2,0,6,5,1,7,3,4}

// 	var dlx_x = x ^ dlx
// 	var dly = sbox[x] ^ sbox[dlx_x]


// 	fmt.Println(dly)
// }

package main
import "fmt"

func main(){
	sbox := [8]int{2,0,6,5,1,7,3,4}

	for dlx:=0; dlx < len(sbox) ; dlx++ {

		counts := [8]int{0,0,0,0,0,0,0,0}

		for x:=0 ; x < len(sbox); x++ {

			var dlx_x = x ^ dlx
			var dly = sbox[x] ^ sbox[dlx_x]
			fmt.Print(dly)

			counts[dly] += 1

		}
		fmt.Print("   ")
		for i := 0; i < len(counts); i++ {
			zoya := float32(counts[i]) / float32(len(sbox))

			fmt.Print(" ",zoya," ")
		}
		fmt.Print("  ")
		fmt.Print("counts " ,counts)
		fmt.Println("")
	}
}