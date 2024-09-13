const std = @import("std");
const http = std.http;

const URL_ADDRESS = "https://adventofcode.com/2023/day/1";
const COOKIE_VALUE = "session=53616c7465645f5f0fb05a638f26c3836be159a19a930638406a8e6ef4b42d9d20672bff40289036eec622fac4e3796f12e69ec18ff738e6382ae402e9b3347e";
pub fn main() !void {
    sayHello("day 1");
    // Create an allocator.
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const message = readApi(allocator);
    std.log.info("{any}\n", .{message});
}

fn sayHello(day: []const u8) void {
    std.debug.print("Welcome to the {s} of advent of code 2023\n", .{day});
}

fn readApi(allocator: std.mem.Allocator) ![]u8 {
    var client = http.Client{ .allocator = allocator };
    const uri = try std.Uri.parse(URL_ADDRESS);

    // release client resources
    defer client.deinit();

    // const headers = std.http.Client.Request.Headers{};
    const header = http.Header{ .name = "cookie", .value = COOKIE_VALUE };
    const headers = std.ArrayList(http.Header).init(allocator);
    defer headers.deinit();
    try headers.append(header);
    client.open(std.http.Method.GET, uri, .{ .extra_headers = &headers });

    // Connect to server
    var request = try client.connect(URL_ADDRESS, 443, std.http.Client.Connection.Protocol.tls);
    defer request.close(allocator);

    // Send request to the server
    // request.start();
    const buffer: []u8 = undefined;
    _ = try request.read(buffer);

    // Return body
    return request.reader().readAllAlloc(allocator, 8192);
}
