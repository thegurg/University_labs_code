
// dly = S(dlx ^ x) ^ S(x)
// x ^ dlx
// S(x1) + s(x2)

// fn main(){
    
//     let x = 5;
//     let  dlx = 3;

//     let sbox = [2,0,6,5,1,7,3,4];

//     let dlx_x = x ^ dlx;
//     let dly = sbox[x] ^ sbox[dlx_x];

//     print!("{}" , dly);

// }


fn ddt_calculate(sbox: &[usize; 8]){
    println!("dlx/dly");
    for dlx in 0..sbox.len() {

        let mut counts = [0; 8];
        
            for x in 0..sbox.len() {

                let dly = sbox[x] ^ sbox[x ^ dlx];

                print!("{:>4}", dly);

                counts[dly] += 1;
                

            }
            print!("  ");

            for i in 0..sbox.len(){

                let zoya = counts[i] as f64 / sbox.len() as f64;

                print!(" {:>4} ", zoya)

            }
            print!("   ");
    println!(" counter {:?}", counts);
    }
}

fn main(){
    let sbox1 = [2,0,6,5,1,7,3,4];
    let sbox2 = [5,4,7,0,6,3,1,2];

    ddt_calculate(&sbox1 );
    println!("-------------------------------------");
    ddt_calculate(&sbox2 );

}



// fn main(){

//     let dlx = 3;

//     let mut counts = [0; 8];


//     let sbox = [2,0,6,5,1,7,3,4];


//     for x in 0..sbox.len() {

//         let dly = sbox[x] ^ sbox[x ^ dlx];

//         println!("{}", dly);

//         counts[dly] += 1;
        

//     }
//     println!("{:?}", counts);
// }