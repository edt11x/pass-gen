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
    if length >= 24 { score += 1; }
    
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());
    
    if has_upper { score += 1; }
    if has_lower { score += 1; }
    if has_digit { score += 1; }
    if has_symbol { score += 1; }
    
    match score {
        0..=3 => ("Weak", Color::from_rgb_u8(255, 0, 0)),
        4..=5 => ("Medium", Color::from_rgb_u8(255, 165, 0)),
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
            let format = ui.get_selected_format();
            
            let charset = match format.as_str() {
                "Alpha" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
                "Alphanumeric" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
                "Base36" => "abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
                "Base58" => "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string(),
                "Base62" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string(),
                "Hex (Lower)" => "0123456789abcdef".to_string(),
                "Hex (Upper)" => "0123456789ABCDEF".to_string(),
                "Numeric" => "0123456789".to_string(),
                _ => {
                    let mut s = "abcdefghijklmnopqrstuvwxyz".to_string();
                    if ui.get_use_uppercase() {
                        s.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                    }
                    if ui.get_use_numbers() {
                        s.push_str("0123456789");
                    }
                    if ui.get_use_symbols() {
                        s.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
                    }
                    s
                }
            };

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
