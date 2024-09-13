const std = @import("std");
const net = std.net;
const testing = std.testing;

const Result = struct {
    connection: net.Server.Connection,
    connection_count: u32,
};

pub fn handleConnection(conn: net.Server.Connection, stdout: std.fs.File.Writer) !void {
    defer conn.stream.close();

    var buf: [1024]u8 = undefined;

    const bytes = try conn.stream.read(&buf);
    try stdout.print("[INFO] Received {d} bytes from client - {s}\n", .{ bytes, buf[0..bytes] });

    _ = try conn.stream.write(
        \\HTTP/1.1 200 OK
        \\Content-Type: text/html; charset=UTF-8
        \\Content-Length: 1000
        \\
        \\<!DOCTYPE html>
        \\<html>
        \\    <head>
        \\        <title>Web server</title>
        \\    </head>
        \\    <body>
        \\        <h1>My Zig web server</h1>
        \\        <p>hello world!</p>
        \\    </body>
        \\</html>
    );
    // _ = try conn.stream.write("Hello from server!");
    try stdout.print("[DEBUG] session closed:\n\n", .{});
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const address = try net.Address.resolveIp("0.0.0.0", 3000);
    var server = try address.listen(.{
        .reuse_address = true,
    });
    defer server.deinit();

    try stdout.print("[INFO] Server listening on {}\n", .{server.listen_address});

    var session_counter: u32 = 0;
    while (true) {
        var session_details: Result = undefined;
        session_details = try start_connection(&server, &session_counter, stdout);
        const connection_thread = try std.Thread.spawn(.{}, handleConnection, .{ session_details.connection, stdout });
        defer connection_thread.detach();
    }
}

fn start_connection(server: *net.Server, session_index: *u32, stdout: std.fs.File.Writer) !Result {
    const connection = try server.accept();
    errdefer connection.stream.close(); // If the thread fails to be created
    // const new_index = session_index + 1;
    session_index += 1;
    try log_new_connection(connection, &session_index, stdout);

    return Result{
        .connection = connection,
        .connection_count = undefined,
    };
}

fn log_new_connection(connection: net.Server.Connection, session_index: *u32, stdout: std.fs.File.Writer) !void {
    try stdout.print("[DEBUG] new connection on: {}\n", .{connection.address});
    try stdout.print("[DEBUG] sessions conunt: {}\n", .{session_index});
}
