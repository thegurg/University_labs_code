const std = @import("std");

pub fn ddt_calc(sbox: [8]u8) void {

    var dlx: usize = 0;

    while(dlx < sbox.len) : (dlx += 1) {

        var x: usize = 0;
        var i: usize = 0;

        var counts = [_]u8{0,0,0,0,0,0,0,0};


        while(x < sbox.len) : (x += 1) {

            const dly = sbox[dlx ^ x] ^ sbox[x];

            std.debug.print(" {d}", .{dly});

            counts[dly] += 1;
        }
        std.debug.print("  ", .{});

        while (i < counts.len ) : (i += 1){

            const zoya = @as(f32 , counts[i]) / @as(f32 , sbox.len);

            std.debug.print(" |{d}", .{zoya});
        }
        std.debug.print("   ", .{});

        std.debug.print("{any}\n", .{counts});
    }
}

pub fn main() void {
    const sbox1 = [_]u8{2, 0, 6, 5, 1, 7, 3, 4};
    const sbox2 = [_]u8{5, 4, 7, 0, 6, 3, 1, 2};

    ddt_calc(sbox1);
    std.debug.print("-----------------\n", .{});
    ddt_calc(sbox2);
}

