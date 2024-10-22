#[path = "ttai/text_to_ascii_image.rs"]
mod ttai;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args.iter() {
        let results = ttai::transform(arg);

        for r in results {
            println!("{}", r);
        }
    }
}

