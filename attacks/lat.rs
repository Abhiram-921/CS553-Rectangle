/* LAT
 * Implementation in rust, no dependencies.
 * compiling: $rustc lat.rs
 * execution: $./lat
 */


const SBOX: [u32; 16] = [6, 5, 0xc, 0xa, 1, 0xe, 7, 9, 0xb, 0, 3, 0xd, 8, 0xf, 4, 2];

fn main() {
    print!("  | ");
    for i in 0..16 {
        print!("  {:x} ", i)
    }
    println!();
    // For input i and output j
    for i in 0..16 {
        print!("{:x} | ", i);
        for j in 0..16 {
            let mut cnt = 0;
            for k in 0..16 {
                // inp == input with mask, out == output with mask
                let inp: i32 =  format!("{:b}", (i & k)).to_string().chars().filter(|&x| x == '1').count() as i32;
                let outp: i32 =  format!("{:b}", (SBOX[k] & j)).to_string().chars().filter(|&x| x == '1').count() as i32;
                
                // If XOR of masked input bits is equal to XOR of masked output then cnt++
                if (inp - outp) % 2 == 0 {
                    cnt += 1;
                }
            }
            // Print deviation from 8
            if cnt > 8 {
                print!(" +{} ", cnt - 8)
            }
            else if cnt != 8 {
                print!("{:width$} ", cnt - 8, width = 3)
            }
            else {
                print!("  . ")
            }
        }
        println!();
    }
}

