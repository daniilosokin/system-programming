use gtk4::prelude::*;
use gtk4::{Adjustment, Align, Application, ApplicationWindow, Box, Button, Justification, Label,
           ListBox, Orientation, PolicyType, Scale, ScrolledWindow, VolumeButton};


const TITLE_NAME: &str = "Audio Player";
const STATUS_LIST: &[&str] = &["Is playing", "Is stopped"];


pub struct Control
{
    pub master: Box,
    play_button: Button,
    stop_button: Button,
    prev_button: Button,
    next_button: Button,
    adjust: Adjustment,
    song_progress: Scale,
    volume_button: VolumeButton,
}

impl Control
{
    fn new() -> Control
    {
        let master = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .margin_start(3)
            .margin_end(3)
            .margin_top(3)
            .margin_bottom(3)
            .build();

        let play_button = Button::builder()
            .has_frame(false)
            .icon_name("media-playback-start")
            .margin_start(3)
            .margin_end(3)
            .valign(Align::Center)
            .halign(Align::Center)
            .build();

        let stop_button = Button::builder()
            .has_frame(false)
            .icon_name("media-playback-stop")
            .margin_start(3)
            .margin_end(3)
            .valign(Align::Center)
            .halign(Align::Center)
            .build();

        let prev_button = Button::builder()
            .has_frame(false)
            .icon_name("media-skip-backward")
            .margin_start(3)
            .margin_end(3)
            .valign(Align::Center)
            .halign(Align::Center)
            .build();

        let next_button = Button::builder()
            .has_frame(false)
            .icon_name("media-skip-forward")
            .margin_start(3)
            .margin_end(3)
            .valign(Align::Center)
            .halign(Align::Center)
            .build();

        let adjust = Adjustment::builder()
            .value(0.0)
            .lower(0.0)
            .upper(1.0)
            .build();

        let song_progress = Scale::builder()
            .orientation(Orientation::Horizontal)
            .adjustment(&adjust)
            .margin_start(3)
            .margin_end(3)
            .valign(Align::Center)
            .halign(Align::Fill)
            .hexpand(true)
            .build();

        let volume_button = VolumeButton::builder()
            .value(0.8)
            .margin_start(3)
            .margin_end(3)
            .valign(Align::Center)
            .halign(Align::Center)
            .build();

        master.append(&play_button);
        master.append(&stop_button);
        master.append(&prev_button);
        master.append(&next_button);
        master.append(&song_progress);
        master.append(&volume_button);

        Control
        {
            master,
            play_button,
            stop_button,
            prev_button,
            next_button,
            adjust,
            song_progress,
            volume_button,
        }
    }
}

pub struct Playlist
{
    playlist: ListBox,
    pub master: ScrolledWindow,
}

impl Playlist
{
    fn new() -> Playlist
    {
        let playlist = ListBox::builder()
            .build();

        let master = ScrolledWindow::builder()
            .child(&playlist)
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(PolicyType::Never)
            .vscrollbar_policy(PolicyType::Always)
            .build();

        Playlist
        {
            playlist,
            master,
        }
    }
}

pub struct Status
{
    pub master: Label,
}

impl Status
{
    fn new() -> Status
    {
        let status = Label::builder()
            .label(STATUS_LIST[1])
            .justify(Justification::Left)
            .margin_start(3)
            .margin_end(3)
            .margin_top(3)
            .margin_bottom(3)
            .build();

        Status
        {
            master: status,
        }
    }
}


pub struct AudioPlayer
{
    pub window: ApplicationWindow,
    pub master: Box,
    control: Control,
    playlist: Playlist,
    status: Status,
}

impl AudioPlayer
{
    pub(crate) fn new(app: &Application) -> AudioPlayer
    {
        let control = Control::new();
        let playlist = Playlist::new();
        let status = Status::new();

        let master = Box::new(Orientation::Vertical, 0);
        master.append(&control.master);
        master.append(&playlist.master);
        master.append(&status.master);

        let window = ApplicationWindow::builder()
            .application(app)
            .title(TITLE_NAME)
            .child(&master)
            .width_request(500)
            .height_request(250)
            .build();

        AudioPlayer
        {
            window,
            master,
            control,
            playlist,
            status,
        }
    }
}