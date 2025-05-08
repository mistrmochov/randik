use crate::constants;
use crate::events::events;
use crate::utils::{ConfFile, get_border_color, get_conf_data};
use dirs::home_dir;
use eyre::{Ok, Result, eyre};
use gtk4::prelude::GtkWindowExt;
use gtk4::{
    self as gtk, ApplicationWindow, Box, Builder, Button, Label, Orientation, Popover,
    gdk::Rectangle, glib::object::IsA, prelude::*,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use regex::Regex;

pub fn build_ui(app: &gtk::Application) -> Result<()> {
    let builder = Builder::from_string(constants::UI_XML);

    let window: ApplicationWindow = get_object(&builder, "window")?;
    window.set_application(Some(app));
    if let Some(home) = home_dir() {
        let conf = ConfFile::new(home.join(".config/randik/config.json"))?;
        let layer = get_conf_data(conf.read(), "layer");

        window.init_layer_shell();
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);
        if layer == "top" {
            window.set_layer(Layer::Top);
        } else {
            window.set_layer(Layer::Overlay);
        }
        window.set_keyboard_mode(KeyboardMode::OnDemand);

        events(app.to_owned(), builder)?;

        app.connect_activate(move |_| {
            window.present();
            window.set_decorated(false);
        });
    }

    Ok(())
}

pub fn get_object<T>(builder: &Builder, name: &str) -> Result<T>
where
    T: IsA<gtk4::glib::Object>,
{
    builder.object(name).ok_or(eyre!(
        "Unable to get UI element {}, this likely means the XML was changed/corrupted.",
        name
    ))
}

pub fn change_border_color(css: String) -> Result<String> {
    let home = home_dir().expect("Couldn't find home directory");
    let conf = ConfFile::new(home.join(".config/randik/config.json"))?;
    let mut colors_vec_string = get_border_color(conf.read());
    if colors_vec_string.is_empty() || (colors_vec_string.len() != 3) {
        println!("The color of the border is empty or is in a wrong format, going with default");
        if !colors_vec_string.is_empty() {
            colors_vec_string.clear();
        }
        for _i in 1..=3 {
            colors_vec_string.push("255".to_string());
        }
    } else {
        // let mut colors_vec_string_clone = colors_vec_string.clone();
        for item in colors_vec_string.clone().iter() {
            if item.chars().count() > 3 {
                println!("The color is in wrong format, going with default!");
                colors_vec_string.clear();
                for _i in 1..=3 {
                    colors_vec_string.push("255".to_string());
                }
                break;
            }
        }
    }

    let regex_from = Regex::new(r"@define-color border .*")?;
    let to = format!(
        "@define-color border rgb({}, {}, {});",
        colors_vec_string[0].to_string(),
        colors_vec_string[1].to_string(),
        colors_vec_string[2].to_string()
    );
    let new_css = regex_from.replace(&css, to).to_string();
    Ok(new_css)
}

pub fn create_popover(message: &str, problem: &str, builder: Builder) -> Result<()> {
    let window: ApplicationWindow = get_object(&builder, "window")?;
    let popup = Popover::builder().has_arrow(false).build();
    let title = Label::new(Some(&message));
    title.add_css_class("popup_title");
    let details = Label::new(Some(&problem));
    let button = Button::builder().label("Ok").build();
    button.add_css_class("suggested-action");

    let hbox = Box::new(Orientation::Horizontal, 0);
    hbox.append(&button);
    hbox.set_halign(gtk4::Align::End);

    let vbox = Box::new(Orientation::Vertical, 20);
    vbox.append(&title);
    vbox.append(&details);
    vbox.append(&hbox);
    vbox.set_valign(gtk4::Align::Center);
    vbox.add_css_class("popup_vbox");

    let x = window.width() / 2;
    let y = (window.height() / 2) - 50;
    let rect = Rectangle::new(x, y, 1, 1);
    popup.set_child(Some(&vbox));
    popup.set_parent(&window);
    popup.set_pointing_to(Some(&rect));
    let popup_clone = popup.clone();

    button.connect_clicked(move |_| {
        popup_clone.popdown();
    });

    popup.popup();
    Ok(())
}
