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
            let r = i / (image_width-1);
            let g = (image_height-1-j) / (image_height-1);
            let b = 0.25;

            let ir = 255.999 * r as f32;
            let ig = 255.999 * g as f32;
            let ib = 255.999 * b as f32;

            //std::cout << ir << ' ' << ig << ' ' << ib << '\n';
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
