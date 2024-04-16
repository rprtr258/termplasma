const std = @import("std");
const math = std.math;
const write = std.os.write;
const time = std.time;

fn max(a: f64, b: f64) f64 {
	return if (a > b) a else b;
}

const Buffer = struct {
	const max_term_height = 332;
	const max_term_width = 1914;
	const bytes_per_char = 24;
	const max_buf_size = max_term_height * max_term_width * bytes_per_char;
	buf: [max_buf_size]u8,
	i: usize,

	fn new() Buffer {
		return Buffer{
			.buf = [_]u8{0} ** (max_buf_size),
			.i = 0,
		};
	}

	fn reset(self: *Buffer) void {
		self.*.i = 0;
	}

	fn get(self: *Buffer) []const u8 {
		return self.buf[0..self.i];
	}

	fn write_bytes(self: *Buffer, bs: []const u8) void {
		std.mem.copy(u8, self.buf[self.i..], bs);
		self.i += bs.len;
	}

	fn write_int(self: *Buffer, x: u8) void {
		if (x < 10) {
			self.write_bytes(&[_]u8{'0' + x});
		} else if (x < 100) {
			self.write_bytes(&[_]u8{'0' + x/10, '0' + x%10});
		} else { // 100 <= x < 256
			self.write_bytes(&[_]u8{'0' + x/100, '0' + (x/10)%10, '0' + x%10});
		}
	}
};

fn draw(buf: *Buffer, w: usize, h: usize, t: f64) void {
	buf.reset();
	var y: isize = 0;
	while (y < h) : (y += 1) {
		var x: isize = 0;
		while (x < w) : (x += 1) {
			const uvx: f64 = @as(f64, @floatFromInt(x)) / @as(f64, @floatFromInt(w));
			const uvy: f64 = @as(f64, @floatFromInt(y)) / @as(f64, @floatFromInt(h));

			const v1: f64 = math.sin(uvx*5.0 + t);
			const v2: f64 = math.sin(5.0*(uvx*math.sin(t/12.0)+uvy*math.cos(t/13.0)) + t);

			const cx: f64 = uvx + math.sin(t/15.0)*5.0;
			const cy: f64 = uvy + math.sin(t/13.0)*5.0;
			const v3: f64 = math.sin(math.sqrt(100.0*(cx*cx+cy*cy)) + t);

			const vf: f64 = v1 + v2 + v3;
			const r: u8 = @as(u8, @intFromFloat(max(0, math.cos(vf*math.pi)-0.5) * 2.0 * 255.0));
			const g: u8 = @as(u8, @intFromFloat(max(0, math.sin(vf*math.pi+6.0*math.pi/3.0)-0.5) * 2.0 * 255.0));
			const b: u8 = @as(u8, @intFromFloat(max(0, math.sin(vf*math.pi+4.0*math.pi/3.0)-0.5) * 2.0 * 255.0));

			buf.write_bytes(&[_]u8{'\x1b', '[', '4', '8', ';', '2', ';'});
			buf.write_int(r);
			buf.write_bytes(&[_]u8{';'});
			buf.write_int(g);
			buf.write_bytes(&[_]u8{';'});
			buf.write_int(b);
			buf.write_bytes(&[_]u8{'m', ' '});
		}
	}
}

fn getwinsize() struct{w: usize, h: usize} {
	var ws: std.os.linux.winsize = undefined;
	_ = std.os.linux.ioctl(2, std.os.linux.T.IOCGWINSZ, @intFromPtr(&ws));
	return .{.w = ws.ws_col, .h = ws.ws_row};
}

pub fn main() !void {
	const hide_cursor = "\x1b[?25l";
	_ = try write(2, hide_cursor);

	var buf = Buffer.new();
	var tv = try time.Timer.start();
	var prev = tv.read();
	while (true) {
		const now = tv.read();
		var elapsed = tv.read() - prev;
		const NS_PER_FRAME = 1000000000/60;
		if (elapsed < NS_PER_FRAME) {
			std.time.sleep(NS_PER_FRAME - elapsed);
			continue;
		}
		prev = now;

		const size = getwinsize();
		const t = @as(f64, @floatFromInt(tv.previous.since(tv.started))) / 1e9;
		draw(&buf, size.w, size.h, t);

		const move_topleft = "\x1b[0;0H\x1b[?2026h";
		_ = try write(1, move_topleft);
		_ = try write(1, buf.get());
		const end_frame = "\x1b[?2026l";
		_ = try write(1, end_frame);
	}
	return 0;
}
