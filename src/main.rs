slint::include_modules!();

use rand::Rng;
use arboard::Clipboard;
use slint::{Color, SharedString};

fn calculate_strength(password: &str) -> (&'static str, Color) {
    if password.is_empty() {
        return ("None", Color::from_rgb_u8(128, 128, 128));
    }
    
    let mut score = 0;
    let length = password.len();
    
    if length >= 12 { score += 1; }
    if length >= 16 { score += 1; }
    
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());
    
    if has_upper && has_lower { score += 1; }
    if has_digit { score += 1; }
    if has_symbol { score += 1; }
    
    match score {
        0..=2 => ("Weak", Color::from_rgb_u8(255, 0, 0)),
        3..=4 => ("Medium", Color::from_rgb_u8(255, 165, 0)),
        _ => ("Strong", Color::from_rgb_u8(0, 128, 0)),
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_generate_password({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let length = ui.get_length() as usize;
            let use_uppercase = ui.get_use_uppercase();
            let use_numbers = ui.get_use_numbers();
            let use_symbols = ui.get_use_symbols();

            let mut charset = "abcdefghijklmnopqrstuvwxyz".to_string();
            if use_uppercase {
                charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }
            if use_numbers {
                charset.push_str("0123456789");
            }
            if use_symbols {
                charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
            }

            let mut rng = rand::thread_rng();
            let password: String = (0..length)
                .map(|_| {
                    let idx = rng.gen_range(0..charset.len());
                    charset.chars().nth(idx).unwrap()
                })
                .collect();

            let (strength_text, strength_color) = calculate_strength(&password);
            
            ui.set_password(password.into());
            ui.set_strength(SharedString::from(strength_text));
            ui.set_strength_color(strength_color);
        }
    });

    ui.on_copy_to_clipboard({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let password = ui.get_password();
            if !password.is_empty() {
                if let Ok(mut clipboard) = Clipboard::new() {
                    let _ = clipboard.set_text(password.to_string());
                }
            }
        }
    });

    ui.on_clear_password({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_password("".into());
            let (strength_text, strength_color) = calculate_strength("");
            ui.set_strength(SharedString::from(strength_text));
            ui.set_strength_color(strength_color);
        }
    });

    ui.run()
}
