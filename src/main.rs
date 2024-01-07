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

fn main() -> io::Result<()> {
    let hide_cursor = b"\x1b[?25l";
    io::stdout().write_all(hide_cursor)?;

    let mut t0 = SystemTime::now();

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
        let buf = (0..h)
            .into_iter()
            .flat_map(|y| (0..w)
                .into_iter()
                .flat_map(move |x| {
                    let uvx = (x as f64) / (w as f64);
                    let uvy = (y as f64) / (h as f64);

                    let v1 = (5.0 * uvx + t).sin();
                    let v2 = (5.0 * (uvx * (t / 12.0).sin() + uvy * (t / 13.0).cos()) + t).sin();

                    let cx = uvx + (t / 15.0).sin() * 5.0;
                    let cy = uvy + (t / 13.0).sin() * 5.0;
                    let v3 = ((100.0 * (cx * cx + cy * cy)).sqrt() + t).sin();

                    let vf = v1 + v2 + v3;

                    [b'\x1b',b'[',b'4',b'8',b';',b'2']
                        .into_iter()
                        .chain(
                            [
                                (vf * MATH_PI                      ).cos(),
                                (vf * MATH_PI + 6.0 * MATH_PI / 3.0).sin(),
                                (vf * MATH_PI + 4.0 * MATH_PI / 3.0).sin(),
                            ]
                                .into_iter()
                                .map(|c| ((c-0.5).max(0.0) * 2.0 * 255.0) as u8)
                                .flat_map(|x|
                                    [b';'].into_iter().chain(
                                    if x < 10 {
                                        vec![x]
                                    } else if x < 100 {
                                        vec![x/10, x%10]
                                    } else {
                                        vec![x/100, x/10%10, x%10]
                                    }
                                        .into_iter()
                                        .map(|c| c as u8 + b'0')
                                    )
                                )
                        )
                        .chain([b'm', b' '].into_iter())
                }))
            .collect::<Vec<u8>>();

        let move_topleft = b"\x1b[0;0H\x1b[?2026h";
        io::stdout().write_all(move_topleft)?;
        io::stdout().write_all(&buf)?;
        let end_frame = b"\x1b[?2026l";
        io::stdout().write_all(end_frame)?;
        io::stdout().flush()?;
    }
}
