use crate::ui::{create_popover, get_object};
use crate::utils::string_to_i64;
use eyre::{Ok, Result};
use gtk4::{
    self as gtk, Builder, Button, Entry, Label, glib::ControlFlow, glib::timeout_add_local,
    prelude::*,
};
use rand::Rng;
use std::cell::RefCell;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

pub fn events(app: gtk::Application, builder: Builder) -> Result<()> {
    let exit_button: Button = get_object(&builder, "exit-button")?;
    let gen_button: Button = get_object(&builder, "gen-button")?;
    let to_entry: Entry = get_object(&builder, "to-entry")?;
    let from_entry: Entry = get_object(&builder, "from-entry")?;
    let result_label: Label = get_object(&builder, "result-label")?;
    let to_error = Rc::new(RefCell::new(false));
    let from_error = Rc::new(RefCell::new(false));
    let number_animate_error = Rc::new(RefCell::new(false));

    exit_button.connect_clicked(move |_| {
        app.quit();
    });

    let lock = "/tmp/randik.lock";
    gen_button.connect_clicked(move |_| {
        if !PathBuf::from(lock).exists() {
            let _lock_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(lock)
                .expect("Failed to create lock file.");
            if (to_entry.text().is_empty() || to_entry.text() == "")
                || (from_entry.text().is_empty() || from_entry.text() == "")
            {
                let msg;
                let problem;
                if (to_entry.text().is_empty() || to_entry.text() == "")
                    && (from_entry.text().is_empty() || from_entry.text() == "")
                {
                    msg = "Both values are empty.";
                    problem = "The both values are empty, can't continue!";
                } else if from_entry.text().is_empty() || from_entry.text() == "" {
                    msg = "\"From\" value is empty.";
                    problem = "The \"from\" value is empty, can't continue!";
                } else {
                    msg = "\"To\" value is empty.";
                    problem = "The \"to\" value is empty, can't continue!";
                }
                create_popover(msg, problem, builder.clone()).expect("Failed to create Popover.");
                fs::remove_file(lock).expect("Failed to remove lock file.");
            } else {
                let from = string_to_i64(from_entry.text().to_string(), from_error.clone());
                let mut to = string_to_i64(to_entry.text().to_string(), to_error.clone());
                if *to_error.borrow() || *from_error.borrow() {
                    let msg;
                    let problem;
                    if *to_error.borrow() && *from_error.borrow() {
                        msg = "Both values are wrong.";
                        problem = format!(
                            "Values \"{}\", \"{}\" are not numbers!",
                            from_entry.text(),
                            to_entry.text()
                        );
                    } else if *from_error.borrow() {
                        msg = "From value is wrong.";
                        problem = format!("Value \"{}\" is not number!", from_entry.text());
                    } else {
                        msg = "To value is wrong.";
                        problem = format!("Value \"{}\" is not number!", from_entry.text());
                    }
                    create_popover(msg, problem.as_str(), builder.clone())
                        .expect("Failed to create Popover.");
                    fs::remove_file(lock).expect("Failed to remove lock file.");
                } else {
                    if from >= to {
                        to = from + 1;
                        to_entry.set_text(to.to_string().as_str());
                    }
                    let result_number = rand::rng().random_range(from..=to);
                    let mut number_animate = 0 as i64;

                    if !result_label.label().is_empty() && (result_label.label() != "") {
                        number_animate = string_to_i64(
                            result_label.label().to_string(),
                            number_animate_error.clone(),
                        );
                    } else {
                        result_label.set_label(number_animate.to_string().as_str());
                    }
                    if !*number_animate_error.borrow() {
                        let result_label = result_label.clone();
                        let difference = (result_number - number_animate).abs();

                        let duration = if difference <= 10 {
                            Duration::from_millis(30)
                        } else if difference <= 50 {
                            Duration::from_millis(20)
                        } else if difference <= 250 {
                            Duration::from_millis(8)
                        } else if difference <= 1000 {
                            Duration::from_millis(3)
                        } else {
                            Duration::from_millis(1)
                        };
                        timeout_add_local(duration, move || {
                            if number_animate > result_number {
                                number_animate -= 1;
                                result_label.set_label(number_animate.to_string().as_str());
                                ControlFlow::Continue
                            } else if number_animate < result_number {
                                number_animate += 1;
                                result_label.set_label(number_animate.to_string().as_str());
                                ControlFlow::Continue
                            } else {
                                result_label.set_label(result_number.to_string().as_str());
                                fs::remove_file(lock).expect("Failed to remove lock file.");
                                ControlFlow::Break
                            }
                        });
                    } else {
                        result_label.set_label(result_number.to_string().as_str());
                        fs::remove_file(lock).expect("Failed to remove lock file.");
                    }
                }
            }
        }
    });

    Ok(())
}
