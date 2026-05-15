slint::include_modules!();

use rand::Rng;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_generate_password({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let length = 16;
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

            ui.set_password(password.into());
        }
    });

    ui.run()
}
