use std::mem::MaybeUninit;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant, UNIX_EPOCH, SystemTime};
use libc::{c_ushort, STDOUT_FILENO, TIOCGWINSZ};

const MATH_PI: f64 = 3.14159265358979323846;

#[repr(C)]
struct Winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

fn max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn append_int(i: &mut usize, x: u8, buf: &mut [u8]) {
    if x < 10 {
        buf[*i] = (x + b'0') as char as u8;
        *i += 1;
    } else if x < 100 {
        buf[*i] = (x / 10 + b'0') as char as u8;
        *i += 1;
        buf[*i] = (x % 10 + b'0') as char as u8;
        *i += 1;
    } else {
        buf[*i] = (x / 100 + b'0') as char as u8;
        *i += 1;
        buf[*i] = ((x / 10) % 10 + b'0') as char as u8;
        *i += 1;
        buf[*i] = (x % 10 + b'0') as char as u8;
        *i += 1;
    }
}

fn main() -> io::Result<()> {
    let hide_cursor = b"\x1b[?25l";
    io::stdout().write_all(hide_cursor)?;

    let mut t0 = SystemTime::now();

    let mut buf = vec![b' '; 332 * 1914 * 24];

    loop {
        let tt = SystemTime::now();
        let dura = tt.duration_since(t0).unwrap();
        if dura < Duration::from_millis(1000 / 60) {
            sleep(Duration::from_millis(1000 / 60) - dura);
            continue;
        }
        t0 = tt;

        let mut ws = MaybeUninit::<Winsize>::uninit();
        unsafe { libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) };
        let wss = unsafe {ws.assume_init()};

        let h = wss.ws_row as usize;
        let w = wss.ws_col as usize;

        let mut i = 0;
        let t = (t0.duration_since(UNIX_EPOCH).unwrap().as_millis() as f64)/1000.;
        for y in 0..h {
            for x in 0..w {
                let uvx = (x as f64) / (w as f64);
                let uvy = (y as f64) / (h as f64);

                let v1 = (5.0 * uvx + t).sin();
                let v2 = (5.0 * (uvx * (t / 12.0).sin() + uvy * (t / 13.0).cos()) + t).sin();

                let cx = uvx + (t / 15.0).sin() * 5.0;
                let cy = uvy + (t / 13.0).sin() * 5.0;
                let v3 = ((100.0 * (cx * cx + cy * cy)).sqrt() + t).sin();

                let vf = v1 + v2 + v3;
                let r = (max(0.0, (vf * MATH_PI).cos() - 0.5) * 2.0 * 255.0) as u8;
                let g = (max(0.0, (vf * MATH_PI + 6.0 * MATH_PI / 3.0).sin() - 0.5) * 2.0 * 255.0) as u8;
                let b = (max(0.0, (vf * MATH_PI + 4.0 * MATH_PI / 3.0).sin() - 0.5) * 2.0 * 255.0) as u8;

                buf[i] = b'\x1b';
                i += 1;
                buf[i] = b'[';
                i += 1;
                buf[i] = b'4';
                i += 1;
                buf[i] = b'8';
                i += 1;
                buf[i] = b';';
                i += 1;
                buf[i] = b'2';
                i += 1;
                buf[i] = b';';
                i += 1;

                append_int(&mut i, r, &mut buf);

                buf[i] = b';';
                i += 1;

                append_int(&mut i, g, &mut buf);

                buf[i] = b';';
                i += 1;

                append_int(&mut i, b, &mut buf);

                buf[i] = b'm';
                i += 1;
                buf[i] = b' ';
                i += 1;
            }
        }

        let move_topleft = b"\x1b[0;0H";
        io::stdout().write_all(move_topleft)?;
        io::stdout().write_all(&buf[..i])?;
        io::stdout().flush()?;
    }
}
