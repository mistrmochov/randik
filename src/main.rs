mod constants;
mod events;
mod ui;
mod utils;
use crate::constants::*;
use crate::ui::{build_ui, change_border_color};
use crate::utils::ConfFile;
use dirs::home_dir;
use gtk4::{self as gtk, CssProvider, gdk::Display, prelude::*};
use std::fs::{self, File};
use std::io;

fn files_init() -> io::Result<()> {
    if let Some(home) = home_dir() {
        let randik = home.join(".config/randik");
        let conf = randik.join("config.json");
        if !randik.exists() || !randik.is_dir() {
            println!("Creating {} directory.", randik.to_string_lossy());
            if !home.join(".config").exists() {
                fs::create_dir_all(randik)?;
            } else {
                fs::create_dir(randik)?;
            }
        }

        if !conf.exists() || !conf.is_file() {
            println!("Creating {}", conf.to_string_lossy());
            File::create(conf.clone())?;
            fs::write(conf, DEFAULT_JSON)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let application = gtk::Application::builder()
        .application_id("com.randik.com")
        .build();
    files_init()?;

    application.connect_startup(|app| {
        let provider = CssProvider::new();
        if let Some(home) = home_dir() {
            let mut css = CSS.to_string();
            let css_path = home.join(".config/randik/style.css");
            if css_path.exists() && css_path.is_file() {
                let new_css = ConfFile::new(css_path).expect("Failed to create ConfFile.");
                css = format!("{}\n{}", css, new_css.read());
            }

            css = change_border_color(css).expect("Failed to change the border color.");
            provider.load_from_string(&css);

            gtk::style_context_add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            build_ui(app).expect("Failed to build_ui");
        }
    });

    application.run();
    Ok(())
}
