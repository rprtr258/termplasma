#include "math.h"
#include "unistd.h"
#include "sys/time.h"
#include "sys/ioctl.h"
#include "stdint.h"

struct {
	int16_t Row;
	int16_t Col;
	int16_t Xpixel;
	int16_t Ypixel;
} ws;

char buf[332*1914*24]; // max_term_height*max_term_width*bytes_per_char

struct timezone tz;

#define MATH_PI 3.14159265358979323846

double max(double a, double b) {
	return a > b ? a : b;
}

void append_int(int* i, uint8_t x) {
	if (x < 10) {
		buf[*i] = x + '0';
		(*i)++;
	} else if (x < 100) {
		buf[*i] = x/10 + '0';
		(*i)++;
		buf[*i] = x%10 + '0';
		(*i)++;
	} else { // 100 <= x < 256
		buf[*i] = x/100 + '0';
		(*i)++;
		buf[*i] = (x/10)%10 + '0';
		(*i)++;
		buf[*i] = x%10 + '0';
		(*i)++;
	}
}

int main() {
	char hide_cursor[] = "\x1b[?25l";
	write(2, hide_cursor, sizeof(hide_cursor));

	struct timeval tv;
	gettimeofday(&tv, &tz);
	int64_t t0 = tv.tv_sec*1000 + tv.tv_usec/1000;
	for (;;) {
		gettimeofday(&tv, &tz);
		long tt = tv.tv_sec*1000 + tv.tv_usec/1000;
		if (tt-t0 < 1000/60) {
			continue;
		}
		t0 = tt;

		ioctl(2, TIOCGWINSZ, &ws);
		int h = ws.Row;
		int w = ws.Col;
		int i = 0;

		double t = (double)t0 / 1000.0;
		for (int y = 0; y < h; y++) {
			for (int x = 0; x < w; x++) {
				double uvx = (double)x / w;
				double uvy = (double)y / h;

				double v1 = sin(uvx*5. + t);
				double v2 = sin(5.*(uvx*sin(t/12.)+uvy*cos(t/13.)) + t);

				double cx = uvx + sin(t/15.)*5.;
				double cy = uvy + sin(t/13.)*5.;
				double v3 = sin(sqrt(100.*(cx*cx+cy*cy)) + t);

				double vf = v1 + v2 + v3;
				uint8_t r = (uint8_t)(max(0, cos(vf*MATH_PI)-0.5) * 2. * 255.);
				uint8_t g = (uint8_t)(max(0, sin(vf*MATH_PI+6.*MATH_PI/3.)-0.5) * 2. * 255.);
				uint8_t b = (uint8_t)(max(0, sin(vf*MATH_PI+4.*MATH_PI/3.)-0.5) * 2. * 255.);

				buf[i++] = '\x1b';
				buf[i++] = '[';
				buf[i++] = '4';
				buf[i++] = '8';
				buf[i++] = ';';
				buf[i++] = '2';
				buf[i++] = ';';

				append_int(&i, r);

				buf[i++] = ';';

				append_int(&i, g);

				buf[i++] = ';';

				append_int(&i, b);

				buf[i++] = 'm';
				buf[i++] = ' ';
			}
		}
		char move_topleft[] = "\x1b[0;0H";
		write(1, move_topleft, sizeof(move_topleft));
		write(1, buf, i);
	}
	return 0;
}
