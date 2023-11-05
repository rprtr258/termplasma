package main

import (
	"math"
	"strconv"
	"syscall"
	"time"

	"golang.org/x/sys/unix"
)

const TIOCGWINSZ = 0x5413

func main() {
	buf := make([]byte, 332*1914*24)
	syscall.Write(1, []byte("\x1b[?25l"))
	t0 := time.Now().UnixMilli()
	for {
		tt := time.Now().UnixMilli()
		dura := time.Duration(tt - t0)
		if dura < 1000/60 {
			time.Sleep(time.Millisecond * (1000/60 - dura))
			continue
		}
		t0 = tt

		ws, _ := unix.IoctlGetWinsize(2, unix.TIOCGWINSZ)
		h := int(ws.Row)
		w := int(ws.Col)
		buf = buf[:0]

		t := float64(tt) / 1000
		for y := 0; y < h; y++ {
			for x := 0; x < w; x++ {
				uvx := float64(x) / float64(w)
				uvy := float64(y) / float64(h)

				v1 := math.Sin(uvx*5 + t)
				v2 := math.Sin(5*(uvx*math.Sin(t/12)+uvy*math.Cos(t/13)) + t)

				cx := uvx + math.Sin(t/15)*5
				cy := uvy + math.Sin(t/13)*5
				v3 := math.Sin(math.Sqrt(100*(cx*cx+cy*cy)) + t)

				vf := v1 + v2 + v3
				r := uint8(max(0, math.Cos(vf*math.Pi+0*math.Pi/1)-0.5) * 2 * 255)
				g := uint8(max(0, math.Sin(vf*math.Pi+6*math.Pi/3)-0.5) * 2 * 255)
				b := uint8(max(0, math.Sin(vf*math.Pi+4*math.Pi/3)-0.5) * 2 * 255)

				buf = append(buf, "\x1b[48;2;"...)
				buf = strconv.AppendInt(buf, int64(r), 10)
				buf = append(buf, ';')
				buf = strconv.AppendInt(buf, int64(g), 10)
				buf = append(buf, ';')
				buf = strconv.AppendInt(buf, int64(b), 10)
				// buf = append(buf, "m \x1b[0m"...)
				buf = append(buf, "m "...)
			}
		}
		syscall.Write(1, []byte("\x1b[0;0H"))
		syscall.Write(1, buf)
	}
}
