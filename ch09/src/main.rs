use ch09::GrayscaleMap;

fn main() {
    let width = 1024;
    let height = 576;

    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    println!("{image:?}");
}
