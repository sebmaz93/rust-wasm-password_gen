use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
pub struct PasswordConfig {
    include_lowercase: bool,
    include_uppercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    max_length: usize,
}

#[wasm_bindgen]
pub fn generate_random_password(config: PasswordConfig) -> String {
    let mut combined_charset = String::new();

    if config.include_lowercase {
        combined_charset += "abcdefghijklmnopqrstuvwxyz";
    }
    if config.include_uppercase {
        combined_charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
    if config.include_numbers {
        combined_charset += "0123456789";
    }
    if config.include_symbols {
        combined_charset += "!@#$%^&*()_+[]{}|;:,.<>?";
    }

    let mut generated_password = String::new();
    for _ in 0..config.max_length {
        let random_index = (js_sys::Math::random() * (combined_charset.len() as f64)) as usize;
        generated_password.push(combined_charset.chars().nth(random_index).unwrap());
    }

    generated_password
}
