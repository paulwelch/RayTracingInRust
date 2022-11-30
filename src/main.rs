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
            let r: f64 = i as f64 / (image_width-1) as f64;
            let g: f64 = j as f64 / (image_height-1) as f64;
            let b: f64 = 0.25;

            let ir = (255.999 as f64 * r) as i32;
            let ig = (255.999 as f64 * g) as i32;
            let ib = (255.999 as f64 * b) as i32;

            data = ir.to_string() + &" " + &ig.to_string() + &" " + &ib.to_string() + &"\n";
            handle.write(data.as_bytes())?;
        }
    }

    handle.flush()?;
    Ok(())
}
