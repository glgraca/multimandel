use num_complex::Complex;
use image::{ImageBuffer, RgbImage};

fn _mandel(power: Complex<f32>) -> RgbImage {
   let imgx = 1024;
    let imgy = 1024;
    let dx = 4.0 / imgx as f32;
    let dy = 4.0 / imgy as f32;
    let x0=-2.0;
    let y0=-2.0;
    
    let mut img: RgbImage = ImageBuffer::new(imgx, imgy);
    
    let palette=vec![[0,0,0],[0x55, 0x41,0x5f],[0x64,0x69,0x64],[0xd7,0x73,0x55],[0x50,0x8c,0xd7],[0x64,0xb9,0x64],[0xe6,0xc8,0x6e],[0xdc,0xf5,0xff]];
    
    let mut cx=x0;
    for x in 0..imgx {
        let mut cy=y0;
        for y in 0..imgy {
            let c = Complex::new(cx, cy);
            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < 256 && z.norm() <= 2.0 {
                z = z.powc(power) + c;
                i += 1;
            }
            let pixel = img.get_pixel_mut(x, y);
            if i<256 {
                *pixel = image::Rgb(palette.get(1+i%7).unwrap().clone());
            } else {
                *pixel = image::Rgb([0,0,0]);
            }
            
            cy=cy+dy;
        }
        cx=cx+dx;
    }
    
    img
}

fn _julia(power: Complex<f32>, c: Complex<f32>) -> RgbImage {
   let imgx = 1024;
    let imgy = 1024;
    let dx = 4.0 / imgx as f32;
    let dy = 4.0 / imgy as f32;
    let x0=-2.0;
    let y0=-2.0;
    
    let mut img: RgbImage = ImageBuffer::new(imgx, imgy);
  
    let mut cx=x0;
    for x in 0..imgx {
        let mut cy=y0;
        for y in 0..imgy {
            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z.powc(power) + c;
                i += 1;
            }
            i = (((i as f32).ln()/(255.0 as f32).ln())*255.0) as u8;
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([((i as u32*7) % 255) as u8, ((i as u32*13)%255) as u8, ((i as u32*19)%255) as u8]);
            
            cy=cy+dy;
        }
        cx=cx+dx;
    }
    
    img
}

//ffmpeg  -i test%05d.png -c:v libx264 -vf fps=25 -pix_fmt yuv420p out3.mp4

fn main() {
    let mut file_no=0;
    
    for i in 0..1000 {
      let a=2.0*std::f32::consts::PI*(i as f32/1000.0);
      let img=_mandel(Complex::new(2.0*a.cos(), 2.0*a.sin()));
    
      let filename=format!("test{:05}.png", file_no);
      file_no+=1;
      img.save(filename).unwrap();
    }
}
