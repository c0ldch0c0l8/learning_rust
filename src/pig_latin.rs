pub fn func(strings: &Vec<String>) -> Vec<String> {
    let mut pig_latin: Vec<String> = Vec::new();

    for string in strings {
        let mut string_chars_iter = string.chars();
        let string_first_letter = string_chars_iter.next().unwrap();

        if String::from("AaEeIiOoUu").contains(string_first_letter) {
            pig_latin.push(format!("{}{}", string, "-hay"));
        } else {
            pig_latin.push(format!("{}{}{}{}", string_chars_iter.as_str(), "-", string_first_letter, "ay"));
        }
    }

    pig_latin
}