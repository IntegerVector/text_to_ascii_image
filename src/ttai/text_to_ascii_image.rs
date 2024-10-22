mod ascii_images_map;

pub fn transform(text: &String) -> [String; 6] {
    let mut result = [
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    ];

    for c in text.chars() { 
        let ascii_img_array = ascii_images_map::get(c);

        for i in 0..ascii_img_array.len() {
            result[i] += ascii_img_array[i];
        }
    }

    result
}

