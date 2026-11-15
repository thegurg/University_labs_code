const std = @import("std");

pub fn main() void {

    const sbox1 = [_]u8{2, 0, 6, 5, 1, 7, 3, 4};

    var x: usize = 0;

    var dlx: usize = 0;
    var i: usize = 0;

    while(dlx < sbox1.len) : (dlx += 1) {

        var counts = [_]u8{0,0,0,0,0,0,0,0};


        while(x < sbox1.len) : (x += 1) {

            const dlx_x = x ^ dlx;
            const dly = sbox1[dlx_x] ^ sbox1[x];

            std.debug.print(" {d}", .{dly});

            counts[dly] += 1;
        }
        while (i < counts.len ) : (i += 1){
            const zoya = counts[i] / sbox1.len;

        }
        
    }
}


// x ^ dlx = dlx_x
// dly = s(x) ^ s(dlx_x)