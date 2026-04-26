// direitos autorais DCrust 16/04/2026
// Port do kernel_mktime original do Linux (Linus Torvalds)
const std = @import("std");

const MINUTE: i64 = 60;
const HOUR:   i64 = 60 * MINUTE;
const DAY:    i64 = 24 * HOUR;
const YEAR:   i64 = 365 * DAY;

const month = [_]i64{
    0,
    DAY * (31),
    DAY * (31 + 29),
    DAY * (31 + 29 + 31),
    DAY * (31 + 29 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30),
};

const Tm = struct {
    tm_sec:  i32,
    tm_min:  i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon:  i32,
    tm_year: i32,
};

fn kernel_mktime(tm: *const Tm) i64 {
    const year: i64 = @as(i64, tm.tm_year) - 70;
    var res: i64 = YEAR * year + DAY * ((year + 1) / 4);

    res += month[@intCast(tm.tm_mon)];

    if (tm.tm_mon > 1 and @mod(year + 2, 4) != 0) {
        res -= DAY;
    }

    res += DAY    * @as(i64, tm.tm_mday - 1);
    res += HOUR   * @as(i64, tm.tm_hour);
    res += MINUTE * @as(i64, tm.tm_min);
    res += @as(i64, tm.tm_sec);

    return res;
}

pub fn main() void {
    const tm = Tm{
        .tm_sec  = 0,
        .tm_min  = 0,
        .tm_hour = 0,
        .tm_mday = 1,
        .tm_mon  = 0,
        .tm_year = 70, // 1970
    };

    const timestamp = kernel_mktime(&tm);
    std.debug.print("Timestamp: {d}\n", .{timestamp});
}
