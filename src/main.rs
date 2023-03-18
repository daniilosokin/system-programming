use gtk::prelude::*;
use gtk::{glib, Application};
mod gui;

const APP_ID: &str = "org.IBAS.AudioPlayer";

fn main() -> glib::ExitCode
{
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(gui::audio_player::build_ui);
    app.run()
}

