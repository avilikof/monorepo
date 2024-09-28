const std = @import("std");
const testing = std.testing;

const DAY_1_INPUT = "data/input.txt";

pub fn main() !void {
    std.debug.print("Starting day one challange\n", .{});
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();

    const raw_file_data = try readFile(allocator, DAY_1_INPUT);
    defer allocator.free(raw_file_data);

    var lines = try splitToLines(raw_file_data);
    defer allocator.free(lines);

    while (lines.next()) |line| {
        std.debug.print("{}", .{line});
    }

    try processByLine(&raw_file_data);

    // try readLines(raw_file_data);

    // var lines = std.mem.splitAny(u8, raw_file_data, "\n");
    // var n: u16 = 1;
    // var m: u32 = 0;
    // var number_list = std.ArrayList(u8).init(allocator);
    // defer number_list.deinit();
    // while (lines.next()) |line| {
    //     const res = readLetter(allocator, line, m);
    //     try number_list.append(try res);
    //     n += 1;
    //     m += 1;
    // }
    // std.debug.print("{d}\n", .{number_list.items});
    // var sum: u16 = 0;
    // for (number_list.items) |num| {
    //     sum += num;
    // }
    // std.debug.print("Last result: {d}\n", .{sum});
}

fn splitToLines(buff: []u8) !std.mem.SplitIterator {
    const lines = std.mem.splitAny(u8, buff, "\n");
    return lines;
}

fn processByLine(data: *const []u8) !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var lines = std.mem.splitAny(u8, data.*, "\n");
    const a: u8 = 0;
    while (lines.next()) |line| {
        std.debug.print("Line: {s}\n", .{line});
        _ = readLetter(allocator, line, a) catch |_err| {
            std.debug.print("error: {}", .{_err});
            return _err;
        };
    }
}

// fn readLines(buff: []u8) !void {
//     for (buff) |some| {
//         const a: []const u8 = &[_]u8{some};
//         if (std.mem.eql(u8, a, "\n")) {
//             std.debug.print("NEWLINE\n", .{});
//         }
//         std.debug.print("Some: {}\n", .{some});
//     }
//     std.debug.print("Last line: {}", .{buff[0]});
// }

fn convertStrToInt(string: []u8) !u8 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer std.debug.assert(gpa.deinit() == .ok);

    var numbers = std.ArrayList(u8).init(allocator);
    defer numbers.deinit();

    for (string) |char| {
        const slice: []const u8 = &[_]u8{char};
        const int = parseStringToInt(slice) catch {
            continue;
        };
        try numbers.append(int);
    }
    std.debug.print("convertion: {}", .{numbers});
}

fn readLetter(allocator: std.mem.Allocator, line: []const u8, line_number: u32) !u8 {
    var number_list = std.ArrayList(u8).init(allocator);
    defer number_list.deinit();

    for (line) |char| {
        const slice: []const u8 = &[_]u8{char};
        const num = parseStringToInt(slice) catch {
            continue;
        };
        try number_list.append(num);
    }
    std.debug.print("Line {}: {d}\n", .{ line_number, number_list.items });
    std.debug.print("First: {d} - Last: {d}\n", .{ number_list.items[0], number_list.items[number_list.items.len - 1] });
    std.debug.print("Combined: {d}\n", .{mergeDigits(number_list.items[0], number_list.items[number_list.items.len - 1])});
    return mergeDigits(number_list.items[0], number_list.items[number_list.items.len - 1]);
}

fn mergeDigits(a: u8, b: u8) u8 {
    return a * 10 + b;
}

fn parseStringToInt(s: []const u8) !u8 {
    return std.fmt.parseInt(u8, s, 10);
}

fn readFile(allocator: std.mem.Allocator, path: *const [14:0]u8) ![]u8 {
    // read file
    std.debug.print("File path: {s}\n", .{path});
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    // put file content to a buffer
    const stat = try file.stat();
    const buff = try file.readToEndAlloc(allocator, stat.size);
    std.debug.print("{s}\n", .{buff});
    return buff;
}

test "test parseStringToInt function" {
    try testValidCases();
    try testInvalidCases();
}

fn testValidCases() !void {
    const valid_cases = [_]struct {
        input: []const u8,
        expected: u8,
    }{
        .{ .input = "42", .expected = 42 },
        .{ .input = "0", .expected = 0 },
        .{ .input = "255", .expected = 255 },
    };

    for (valid_cases) |test_case| {
        const result = try parseStringToInt(test_case.input);
        try std.testing.expectEqual(result, test_case.expected);
    }
}

fn testInvalidCases() !void {
    const invalid_cases = [_][]const u8{
        "abc", // Non-numeric characters
        "", // Empty string
        "256", // Out of range for u8
        "-1", // Negative number
    };

    for (invalid_cases) |input| {
        _ = parseStringToInt(input) catch |_err| {
            try std.testing.expectError(error.ParseIntError, .{_err});
        };
        // try std.debug.assert(false, "Expected failure but got {d}", .{result});
        // std.testing.expectError(expected_error: anyerror, actual_error_union: anytype)
    }
}
