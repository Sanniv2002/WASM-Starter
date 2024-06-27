use wasm_bindgen::prelude::*;
use fontdue::Font;

#[wasm_bindgen]
pub fn parse_through_llm(text: &str) -> Vec<String> {
    let font_data = include_bytes!("../llama.ttf") as &[u8];
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();

    // Buffer to store the formatted text
    let mut characters = Vec::new();
    for ch in text.chars() {
        // Rasterize the character to get glyph metrics
        let _ = font.rasterize(ch, 32.0);
        
        // Add basic representation of the glyph to the formatted text
        characters.push(ch.to_string());
    }
    characters
}