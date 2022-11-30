//TODO: write to bytestream redirected to '.ppm' file as image
//use std::io;

fn main() {

    //Image
    let image_width = 256;
    let image_height = 256;

    //Render
    //std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..(image_height-1) {
        for i in 0..(image_width-1) {
            let r: f32 = (i / (image_width-1)) as f32;
            let g: f32 = ((image_height-1-j) / (image_height-1)) as f32;
            let b: f32 = 0.25 as f32;

            let ir = 255.999 as f32 * r;
            let ig = 255.999 as f32 * g;
            let ib = 255.999 as f32 * b;

            //std::cout << ir << ' ' << ig << ' ' << ib << '\n';
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
