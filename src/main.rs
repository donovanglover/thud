use libvips::{ops, VipsImage, VipsApp};

fn main() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");

    app.concurrency_set(2);

    let image = VipsImage::new_from_file("cover.png").unwrap();

    let thumbnail = ops::thumbnail_image(&image, 512).unwrap();

    match ops::vipssave(&thumbnail, "output.png") {
        Err(_) => println!("error: {}", app.error_buffer().unwrap()),
        Ok(_) => println!("Great Success!")
    }
}
