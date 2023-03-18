use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box, Button, CenterBox, Label, ListBox, Orientation,
          PolicyType, ProgressBar, ScrolledWindow, VolumeButton};
use gtk::gio::{MenuItem, Menu};


const TITLE_NAME: &str = "Rust | Audio Player";
const STATUS_LIST: &[&str] = &["Is playing", "Is stopped"];


pub(crate) fn build_ui(app: &Application)
{
    // CONTROL BOX
    let control_box = Box::new(Orientation::Horizontal, 10);
    control_box.set_margin_start(6);
    control_box.set_margin_end(6);
    control_box.set_margin_top(6);
    control_box.set_margin_bottom(6);

    let play_button = Button::with_label("PLAY");
    play_button.set_valign(Align::Center);
    play_button.set_halign(Align::Center);

    play_button.connect_clicked(|button| {
        button.set_label("XDDD");
    });

    let stop_button = Button::with_label("STOP");
    stop_button.set_valign(Align::Center);
    stop_button.set_halign(Align::Center);

    stop_button.connect_clicked(|button| {
        button.set_label("WWWW");
    });

    let prev_button = Button::with_label("PREV");
    prev_button.set_valign(Align::Center);
    prev_button.set_halign(Align::Center);

    let next_button = Button::with_label("NEXT");
    next_button.set_valign(Align::Center);
    next_button.set_halign(Align::Center);

    control_box.append(&play_button);
    control_box.append(&stop_button);
    control_box.append(&prev_button);
    control_box.append(&next_button);

    // SONG PROGRESSBAR
    let song_progress = ProgressBar::new();
    song_progress.set_margin_start(6);
    song_progress.set_margin_end(6);
    song_progress.set_margin_top(6);
    song_progress.set_margin_bottom(6);
    song_progress.set_valign(Align::Center);
    song_progress.set_halign(Align::Fill);
    song_progress.set_hexpand(true);

    // VOLUME BUTTON
    let volume_button = VolumeButton::new();
    volume_button.set_margin_start(6);
    volume_button.set_margin_end(6);
    volume_button.set_margin_top(6);
    volume_button.set_margin_bottom(6);
    volume_button.set_valign(Align::Center);
    volume_button.set_halign(Align::Center);
    volume_button.set_value(0.8);

    // TOOLBAR BOX
    let toolbar = CenterBox::new();
    toolbar.set_start_widget(Some(&control_box));
    toolbar.set_center_widget(Some(&song_progress));
    toolbar.set_end_widget(Some(&volume_button));
    toolbar.set_halign(Align::Fill);
    toolbar.set_hexpand(true);

    // PLAYLIST WINDOW
    let playlist = ScrolledWindow::new();
    playlist.set_hexpand(true);
    playlist.set_vexpand(true);
    playlist.set_hscrollbar_policy(PolicyType::Never);
    playlist.set_vscrollbar_policy(PolicyType::Always);

    let playlist_box = ListBox::new();
    playlist.set_child(Some(&playlist_box));

    // STATUSBAR BOX
    let statusbar = Box::new(Orientation::Horizontal, 0);
    statusbar.set_margin_start(6);
    statusbar.set_margin_end(6);
    statusbar.set_margin_top(6);
    statusbar.set_margin_bottom(6);

    let status = Label::with_mnemonic(STATUS_LIST[1]);
    statusbar.append(&status);

    // MENU BAR
    let add = MenuItem::new(Some("Add"), None);
    let help = MenuItem::new(Some("Help"), None);

    let menu = Menu::new();
    menu.append_item(&add);
    menu.append_item(&help);
    app.set_menubar(Some(&menu));

    // WINDOW
    let window = Box::new(Orientation::Vertical, 0);
    window.append(&toolbar);
    window.append(&playlist);
    window.append(&statusbar);

    // APP WINDOW
    let app_window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE_NAME)
        .child(&window)
        .maximized(true)
        .show_menubar(true)
        .build();
    app_window.present();
}