mod audio_player;
use audio_player::AudioPlayer;

use gtk4::prelude::*;
use gtk4::Application;
use gtk4::glib;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("ssau.giib.AudioPlayer")
        .build();

    application.connect_activate(|app| {
        let audio_player_app = AudioPlayer::new(app);
        audio_player_app.window.show();
    });

    application.run()
}