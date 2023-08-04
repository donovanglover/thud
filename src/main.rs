use libvips::{ops, VipsImage, VipsApp};
use libvips::ops::Interesting;

fn main() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");

    app.concurrency_set(2);

    let image = VipsImage::new_from_file("cover.png").unwrap();

    let options = ops::ThumbnailImageOptions {
        crop: Interesting::Centre,
        ..ops::ThumbnailImageOptions::default()
    };

    ops::thumbnail_image_with_opts(&image, 128, &options).unwrap();
}
