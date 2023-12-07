const std = @import("std");
const testing = std.testing;

fn combinations_to_beat(time: usize, dist: usize) usize {
    var combinations: usize = 0;
    for (0..time) |val| {
        const time_traveled = time - val;
        const dist_traveled = time_traveled * val;
        if (dist < dist_traveled) {
            combinations += 1;
        }
    }
    return combinations;
}

fn solve2(path: []const u8) !void {
    _ = path;
    var time = 0;
    _ = time;
    var dist = 0;
    _ = dist;

    var file = std.fs.openFileAbsolute("", .{});
    defer file.close();
    // TODO: read file lines
    var buf: [4098]u8 = undefined;
    _ = buf;
}

fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "basic add functionality" {
    try testing.expect(add(3, 7) == 10);
}
