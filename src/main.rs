slint::include_modules!();

use rand::Rng;
use arboard::Clipboard;
use slint::{Color, ModelRc, SharedString, VecModel};
use std::rc::Rc;

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
    let history_model = Rc::new(VecModel::<HistoryEntry>::default());
    ui.set_history(ModelRc::from(history_model.clone()));

    ui.on_generate_password({
        let ui_handle = ui.as_weak();
        let history_model = history_model.clone();
        move || {
            let ui = ui_handle.unwrap();
            let length = ui.get_length() as usize;
            let format = ui.get_selected_format();
            let mut rng = rand::thread_rng();
            
            let non_ambiguous = "abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
            
            let password: String = match format.as_str() {
                "Non-ambiguous (12)" => {
                    (0..12).map(|_| {
                        let idx = rng.gen_range(0..non_ambiguous.len());
                        non_ambiguous.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Non-ambiguous" => {
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..non_ambiguous.len());
                        non_ambiguous.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Numeric" => {
                    (0..length).map(|_| rng.gen_range('0'..='9')).collect()
                }
                "Alpha" => {
                    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Alphanumeric" => {
                    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Binary" => {
                    (0..length).map(|_| if rng.gen_bool(0.5) { '0' } else { '1' }).collect()
                }
                "Hex" => {
                    let charset = "0123456789abcdef";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Base 36" => {
                    let charset = "0123456789abcdefghijklmnopqrstuvwxyz";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Base 58" => {
                    let charset = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Base 62" => {
                    let charset = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "ASCII (Printable)" => {
                    (0..length).map(|_| rng.gen_range('!'..='~')).collect()
                }
                "TTY Noise" => {
                    let charset = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    }).collect()
                }
                "Easy to Remember" => {
                    let words = ["alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliett", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu"];
                    let mut p = String::new();
                    let num_words = (length / 6).max(2).min(5);
                    for i in 0..num_words {
                        let mut word = words[rng.gen_range(0..words.len())].to_string();
                        // Simple leet speak
                        word = word.replace('a', "4").replace('e', "3").replace('i', "1").replace('o', "0").replace('s', "5").replace('t', "7");
                        p.push_str(&word);
                        if i < num_words - 1 {
                            p.push(if rng.gen_bool(0.5) { '-' } else { '_' });
                        }
                    }
                    p
                }
                "Work Password" => {
                    // 3 non-ambig + '<' + 3 non-ambig, repeated to make 14
                    // Must contain upper, lower, and numbers.
                    let lower = "abcdefghijkmnopqrstuvwxyz";
                    let upper = "ABCDEFGHJKLMNPQRSTUVWXYZ";
                    let digits = "23456789";
                    
                    let mut base = String::new();
                    // Ensure at least one of each
                    base.push(lower.chars().nth(rng.gen_range(0..lower.len())).unwrap());
                    base.push(upper.chars().nth(rng.gen_range(0..upper.len())).unwrap());
                    base.push(digits.chars().nth(rng.gen_range(0..digits.len())).unwrap());
                    
                    // Shuffle the first 3
                    let mut base_chars: Vec<char> = base.chars().collect();
                    for i in (1..3).rev() {
                        let j = rng.gen_range(0..=i);
                        base_chars.swap(i, j);
                    }
                    
                    let part1: String = base_chars.into_iter().collect();
                    
                    let part2: String = (0..3).map(|_| {
                        let idx = rng.gen_range(0..non_ambiguous.len());
                        non_ambiguous.chars().nth(idx).unwrap()
                    }).collect();
                    
                    let full_base = format!("{}<{}", part1, part2);
                    format!("{}{}", full_base, full_base)
                }
                "Random Integer" => {
                    let min = ui.get_min_range();
                    let max = ui.get_max_range();
                    if min <= max {
                        rng.gen_range(min..=max).to_string()
                    } else {
                        "Invalid Range".to_string()
                    }
                }
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
                    
                    (0..length).map(|_| {
                        let idx = rng.gen_range(0..s.len());
                        s.chars().nth(idx).unwrap()
                    }).collect()
                }
            };

            let (strength_text, strength_color) = calculate_strength(&password);
            
            ui.set_password(password.clone().into());
            ui.set_strength(SharedString::from(strength_text));
            ui.set_strength_color(strength_color);

            // Add to history (most recent first)
            history_model.insert(0, HistoryEntry { password: password.into() });
        }
    });

    ui.on_copy_text_to_clipboard(move |text| {
        if !text.is_empty() {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(text.to_string());
            }
        }
    });

    ui.on_remove_from_history({
        let history_model = history_model.clone();
        move |index| {
            history_model.remove(index as usize);
        }
    });

    ui.run()
}
