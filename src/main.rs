use std::mem::MaybeUninit;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, UNIX_EPOCH, SystemTime};
use libc::{c_ushort, STDOUT_FILENO, TIOCGWINSZ};

const MATH_PI: f64 = 3.14159265358979323846;

#[repr(C)]
struct Winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

struct Buffer {
    buf: Vec<u8>,
    i: usize,
}

impl Buffer {
    fn new(h: usize, w: usize) -> Self {
        Self {
            buf: vec![0; h * w * 24],
            i: 0,
        }
    }
    fn reset(&mut self) {
        self.i = 0;
    }
    fn b(mut self, b: u8) -> Self {
        self.buf[self.i] = b;
        self.i += 1;
        return self
    }
    fn i(self, x: u8) -> Self {
        if x < 10 {
            self.
                b((x + b'0') as char as u8)
        } else if x < 100 {
            self
                .b((x / 10 + b'0') as char as u8)
                .b((x % 10 + b'0') as char as u8)
        } else {
            self
                .b((x / 100 + b'0') as char as u8)
                .b(((x / 10) % 10 + b'0') as char as u8)
                .b((x % 10 + b'0') as char as u8)
        }
    }
}

fn main() -> io::Result<()> {
    let hide_cursor = b"\x1b[?25l";
    io::stdout().write_all(hide_cursor)?;

    let mut t0 = SystemTime::now();

    let mut buf = Buffer::new(332, 1914);

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

        let t = (t0.duration_since(UNIX_EPOCH).unwrap().as_millis() as f64)/1000.;
        buf.reset();
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
                let r = (((vf * MATH_PI                      ).cos() - 0.5).max(0.0) * 2.0 * 255.0) as u8;
                let g = (((vf * MATH_PI + 6.0 * MATH_PI / 3.0).sin() - 0.5).max(0.0) * 2.0 * 255.0) as u8;
                let b = (((vf * MATH_PI + 4.0 * MATH_PI / 3.0).sin() - 0.5).max(0.0) * 2.0 * 255.0) as u8;


                buf = buf
                    .b(b'\x1b').b(b'[').b(b'4').b(b'8').b(b';').b(b'2').b(b';')
                    .i(r)
                    .b(b';')
                    .i(g)
                    .b(b';')
                    .i(b)
                    .b(b'm').b(b' ');
            }
        }

        let move_topleft = b"\x1b[0;0H";
        io::stdout().write_all(move_topleft)?;
        io::stdout().write_all(&buf.buf[..buf.i])?;
        io::stdout().flush()?;
    }
}
