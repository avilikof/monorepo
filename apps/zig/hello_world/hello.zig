const std = @import("std");
const http = std.http;

pub fn main() !void {
    // Create an allocator.
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);
    const allocator = gpa.allocator();

    // Create an HTTP client.
    var client = http.Client{ .allocator = allocator };
    // Release all associated resources with the client.
    defer client.deinit();

    // Parse the URI.
    const uri = std.Uri.parse("https://whatthecommit.com/index.txt") catch unreachable;

    // Create the headers that will be sent to the server.
    var headers = std.http.Header{ .allocator = allocator };
    defer headers.deinit();

    // Accept anything.
    try headers.append("accept", "*/*");

    // Make the connection to the server.
    var request = try client.request(.GET, uri, headers, .{});
    defer request.deinit();

    // Send the request and headers to the server.
    try request.start();

    // Wait for the server to send use a response.
    try request.wait();

    // Read the entire response body, but only allow it to allocate 8KB of memory.
    const body = request.reader().readAllAlloc(allocator, 8192) catch unreachable;
    defer allocator.free(body);

    // Print out the response.
    std.log.info("{s}", .{body});
}
