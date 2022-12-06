use std::io::{self,Write};

fn main() -> io::Result<()> {

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    //Image
    let image_width = 256;
    let image_height = 256;

    //Render
    let mut data = "P3\n".to_string();
    data = data + &image_width.to_string() + &" " + &image_height.to_string() + &"\n255\n";
    handle.write(data.as_bytes())?;

    for j in (0..(image_height)).rev() {
        for i in 0..(image_width) {
            let color = PixelColor { 
                r: ((i as f64 / (image_width-1) as f64) * 255.999 as f64) as i32, 
                g: ((j as f64 / (image_height-1) as f64) * 255.999 as f64) as i32, 
                b: ((0.25 as f64) * 255.999 as f64) as i32,
            };
            write_color(&stdout, color)?;
        }
    }

    handle.flush()?;
    Ok(())
}

struct PixelColor {
   r: i32,
   g: i32,
   b: i32,
}

fn write_color(stdout: &std::io::Stdout, color: PixelColor) -> Result<(), std::io::Error>  {
    // Write the translated [0,255] value of each color component.
    let mut handle = stdout.lock();
    let data = (color.r.to_string()) + &" " + &(color.g.to_string()) + &" " + &(color.b.to_string()) + &"\n";
    handle.write(data.as_bytes())?;
    Ok(())
}