
fn calculate_ddt(sbox: &[usize; 8]) -> [[u32; 8]; 8] {
    let mut ddt = [[0u32; 8]; 8];

    for input_diff in 0..8 {
        for x in 0..8 {
            let output_diff = sbox[x] ^ sbox[x ^ input_diff];
            ddt[input_diff][output_diff] += 1;
        }
    }

    ddt
}

fn print_ddt(name: &str, ddt: &[[u32; 8]; 8]) {
    println!("\n{name}");
    print!("Δin\\Δout");

    for b in 0..8 {
        print!("{:4}", b);
    }
    println!();

    for a in 0..8 {
        print!("{:7}", a);

        for b in 0..8 {
            print!("{:4}", ddt[a][b]);
        }
        println!();
    }

    println!("\nDifferential probabilities:");

    for a in 1..8 {
        for b in 0..8 {
            let count = ddt[a][b];

            if count > 0 {
                println!(
                    "Δin={a}, Δout={b}: pairs={count}, P={:.2}",
                    count as f64 / 8.0
                );
            }
        }
    }
}

fn main() {
    let sbox31 = [2, 0, 6, 5, 1, 7, 3, 4];
    let sbox32 = [5, 4, 7, 0, 6, 3, 1, 2];

    let ddt31 = calculate_ddt(&sbox31);
    let ddt32 = calculate_ddt(&sbox32);

    print_ddt("S-box 31", &ddt31);
    print_ddt("S-box 32", &ddt32);
}
