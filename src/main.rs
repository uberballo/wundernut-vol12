use image;
use image::{GenericImageView, Rgb, RgbImage};
use std::collections::HashSet;

fn get_image_text(filename: String) -> Result<String, std::str::Utf8Error> {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(filename)
        .expect("Error setting image to Leptess.");
    lt.set_source_resolution(700);
    lt.get_utf8_text()
}

fn reveal_hidden_message(filename: String, new_file: &String) -> std::io::Result<()> {
    let image = image::open(filename).expect("File not found.");
    let (w, h) = image.dimensions();
    let mut output = RgbImage::new(w, h);

    for (x, y, mut pixel) in image.pixels() {
        if pixel[2] != 229 {
            output.put_pixel(x, y, Rgb([0, 0, 0]));
        } else {
            output.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }

    output.save(new_file).unwrap();
    Ok(())
}

fn check_for_hidden_messages(filename: String) -> bool {
    let image = image::open(filename).expect("File not found.");
    let mut unique_colors = HashSet::new();

    for (_, _, pixel) in image.pixels() {
        unique_colors.insert(pixel);
    }

    unique_colors.len() > 1
}

fn solve_caesar_cipher(secret_text: String, rotation: u8) -> String {
    let mut res: Vec<char> = vec![' '; secret_text.len()];

    for (i, character) in secret_text.chars().enumerate() {
        res[i] =
            (((((character as u8) - ('A' as u8)) + (26 - rotation)) % 26) + ('A' as u8)) as char
    }
    res.into_iter().collect()
}

fn solve_encrypted_parchment() -> std::io::Result<Vec<String>> {
    let filename = String::from("parchment.png");
    let new_filename = String::from("uusi.png");

    reveal_hidden_message(filename, &new_filename)?;

    let text = get_image_text(new_filename);

    let result = match text {
        Ok(x) => x,
        Err(error) => panic!("{}", error),
    };

    let ciphered: Vec<String> = result
        .split("\n")
        .map(|row| solve_caesar_cipher(String::from(row), 21))
        .collect();

    Ok(ciphered)
}

fn main() {
    if check_for_hidden_messages(String::from("parchment.png")) {
        match solve_encrypted_parchment() {
            Ok(result) => println!("{}", result.join(" ")),
            Err(error) => panic!("{}", error),
        }
    }
}
