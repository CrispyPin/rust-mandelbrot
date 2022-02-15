extern crate png;

use std::ops::{Rem, Sub};

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

const SIZE_X: u16 = 4096;
const SIZE_Y: u16 = 4096;
const ITER_MAX: u16 = 256;
const TOTAL_BYTES: usize = SIZE_X as usize * SIZE_Y as usize * 3;
const PATH: &str = "./test_hue.png";


fn main() {
	save(render());
}

fn render() -> Vec<u8> {
	let mut image: Vec<u8> = Vec::with_capacity(TOTAL_BYTES);
	for y in 0..SIZE_Y {
		for x in 0..SIZE_X {
			let coords = coords(x, y);
			let iter = mandelbrot(coords.0, coords.1);
			let col = color(iter);
			image.push(col[0]);
			image.push(col[1]);
			image.push(col[2]);
		}
	}
	image
}

fn mandelbrot(x0: f32, y0: f32) -> u16 {
	let mut x: f32 = 0.0;
	let mut y: f32 = 0.0;
	let mut iter = 0;
	let mut x2: f32 = 0.0;
	let mut y2: f32 = 0.0;

	while x2 + y2 > 4.0 || iter == ITER_MAX {
		y = 2.0 * x * y + y0;
		x = x2 - y2 + x0;
		x2 = x * x;
		y2 = y * y;
		iter += 1;
	}
	iter
}

fn color(iter: u16) -> [u8; 3] {
	if iter == ITER_MAX {
		return [0,0,0];
	}
	let val = iter as f32 / ITER_MAX as f32;
	hsv2rgb((val * 4.0).fract(), 1.0, 1.0)
}

fn coords(x: u16, y: u16) -> (f32, f32){
	((x as f32 / SIZE_X as f32) * 4.0 - 2.0,
	(y as f32 / SIZE_Y as f32) * 4.0 - 2.0)
}

fn save(image: Vec<u8>) {
	let path = Path::new(PATH);
	let file = File::create(path).unwrap();
	let w = BufWriter::new(file);
	let mut encoder = png::Encoder::new(w, SIZE_X as u32, SIZE_Y as u32);
	encoder.set_color(png::ColorType::Rgb);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header().unwrap();
	writer.write_image_data(&image).unwrap()
}

fn hsv2rgb (h: f32, s: f32, v: f32) -> [u8; 3] {
	// magic from *somewhere*, I forgot
	let hue = h * 6.0;
	let sv = s * v;
	let xf = sv * (1.0 - hue.rem(2.0).sub(1.0).abs());
	let mf = v - sv;

	let c = ((sv + mf) * 255.0) as u8;
	let x = ((xf + mf) * 255.0) as u8;
	let m = (mf * 255.0) as u8;

	match hue as u8 {
		0 => [c, x, m],
		1 => [x, c, m],
		2 => [m, c, x],
		3 => [m, x, c],
		4 => [x, m, c],
		5 => [c, m, x],
		_ => panic!("Hue outside of valid range!")
	}
}
