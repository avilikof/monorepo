const std = @import("std");
// const json = @import("std/json.zig");
// const rand = @import("std/rand.zig");
// const time = @import("std/time.zig");

const AlertState = enum {
    Firing,
    Resolved,
};

const AlertEntity = struct {
    occurrence_id: ?[]u8,
    timestamp: u128,
    description: [256:0]u8,
    state: AlertState,
    alert_id: []u8,

    pub fn random() !AlertEntity {
        return AlertEntity{
            .occurrence_id = null,
            .timestamp = getTimeEpochNano(),
            .description = "random alert",
            .state = getRandomState(),
            .alert_id = try getRandomNumber(),
        };
    }

    pub fn fromBytes(bytes: []const u8) !AlertEntity {
        var deserializer = std.json.Deserializer.init(bytes);
        const entity = try deserializer.deserialize(AlertEntity);
        return convertEmptyStrToNone(entity);
    }

    pub fn asBytes(self: *AlertEntity, allocator: *std.mem.Allocator) ![]u8 {
        const serialized = try std.json.serialize(self);
        return try convertNoneToEmptyString(self, allocator, serialized);
    }

    pub fn getOccurrenceId(self: *AlertEntity) ?[]const u8 {
        return self.occurrence_id;
    }

    pub fn getTimestamp(self: *AlertEntity) u128 {
        return self.timestamp;
    }

    pub fn getDescription(self: *AlertEntity) []const u8 {
        return self.description;
    }

    pub fn getState(self: *AlertEntity) AlertState {
        return self.state;
    }

    pub fn getAlertId(self: *AlertEntity) []const u8 {
        return self.alert_id;
    }

    pub fn setNewOccurrenceId(self: *AlertEntity, allocator: *std.mem.Allocator) !void {
        if (self.occurrence_id == null) {
            self.occurrence_id = try getRandomNumber(allocator, 99999999999999999);
        } else {
            std.debug.warn("Cannot set ID, it already exists: '{}'\n", .{self.occurrence_id});
        }
    }

    pub fn setOccurrenceId(self: *AlertEntity, id: []const u8) void {
        if (self.occurrence_id == null) {
            self.occurrence_id = id;
        } else {
            std.debug.warn("Cannot set ID, it already exists: '{}'\n", .{self.occurrence_id});
        }
    }

    pub fn setState(self: *AlertEntity, state: AlertState) void {
        if (state == self.state) {
            std.debug.warn("Trying to update alert state with same value\n", .{});
        } else {
            self.state = state;
        }
    }
};

fn getTimeEpochNano() u128 {
    const now = try std.time();
    return now.nanos();
}

fn convertEmptyStrToNone(alert: AlertEntity) AlertEntity {
    if (alert.occurrence_id != null and std.mem.eql(u8, alert.occurrence_id.*, "")) {
        alert.occurrence_id = null;
    }
    return alert;
}

fn convertNoneToEmptyString(alert: *AlertEntity, serialized: []u8) ![]u8 {
    if (alert.getOccurrenceId() == null) {
        alert.occurrence_id = "";
    }
    return serialized;
}

fn getRandomState() AlertState {
    const states = [2]AlertState{ AlertState.Firing, AlertState.Resolved };
    var rng = std.rand.DefaultPrng.init(std.crypto.random.bytes(16));
    const random_bool = rng.bool(); // Generates a random boolean (true/false)
    const random_bit: u8 = @intCast(random_bool); // Convert bool to 0 or 1}
    return states[random_bit];
}

fn getRandomNumber() !u8 {
    return 54;
}

pub fn main() !void {
    var alert = try AlertEntity.random();
    const alert_bytes = try alert.asBytes();

    std.debug.print("Serialized Alert: {s}\n", .{alert_bytes});

    const deserialized_alert = try AlertEntity.fromBytes(alert_bytes);

    std.debug.print("Deserialized Alert: {s}\n", .{deserialized_alert});
}
