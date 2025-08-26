#![windows_subsystem = "windows"]
use iced::{
    self,
    widget::{self, pick_list, Button, Checkbox, Column, Row, Text, TextInput},
    window::icon,
    window::Settings,
    Element, Task,
};
mod img;
use img::{get_filename, write_thumbnail};
mod video;
use img::CHAR_IMGS;
use std::path::PathBuf;
use tokio::task;
use video::trim_video;

fn main() -> iced::Result {
    let ico = icon::from_file_data(include_bytes!("icon.ico"), None).expect("Couldn't load icon");
    iced::application("Trimmer", App::update, App::view)
        .window(Settings {
            icon: Some(ico),
            ..Settings::default()
        })
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    End,
    InputFile(String),
    BrowseFile,
    OutputFolder(String),
    BrowserFolder,
    TournamentName(String),
    RoundName(String),
    Date(String),
    Player1(String),
    Fighter1(String),
    Player2(String),
    Fighter2(String),
    StartTime(String),
    EndTime(String),
    UpdateMsg(String),
    GenerateThumbnail(bool),
    GenerateVideo(bool),
    Submit,
    ReloadConfig,
}

#[derive(Clone)]
struct App {
    input_file: String,
    output_folder: String,
    tournament_name: String,
    round_name: String,
    date: String,
    player_1: String,
    fighter_1: String,
    player_2: String,
    fighter_2: String,
    start_time: String,
    end_time: String,
    message: String,
    generate_thumbnail: bool,
    generate_video: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            input_file: String::new(),
            output_folder: String::new(),
            tournament_name: String::new(),
            round_name: String::new(),
            date: String::new(),
            player_1: String::new(),
            fighter_1: CHAR_IMGS.read().expect("Poisoned CHAR_IMGS")[0].clone(),
            player_2: String::new(),
            fighter_2: CHAR_IMGS.read().expect("Poisoned CHAR_IMGS")[0].clone(),
            start_time: String::from("00:00:00"),
            end_time: String::from("00:00:00"),
            message: String::new(),
            generate_thumbnail: true,
            generate_video: true,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::End => return Task::none(),
            Message::InputFile(message) => {
                self.input_file = message;
            }
            Message::BrowseFile => {
                let path = std::env::current_dir().unwrap();
                if let Some(res) = rfd::FileDialog::new()
                    .set_directory(&path)
                    .add_filter(".mp4", &["mp4"])
                    .pick_file()
                {
                    let path = res.to_str().unwrap();
                    self.input_file = path.to_string();
                }
            }
            Message::OutputFolder(message) => {
                self.output_folder = message;
            }
            Message::BrowserFolder => {
                let path = std::env::current_dir().unwrap();
                if let Some(res) = rfd::FileDialog::new().set_directory(&path).pick_folder() {
                    let path = res.to_str().unwrap();
                    self.output_folder = path.to_string();
                }
            }
            Message::TournamentName(message) => {
                self.tournament_name = message;
            }
            Message::RoundName(message) => {
                self.round_name = message;
            }
            Message::Date(message) => {
                self.date = message;
            }
            Message::Player1(message) => {
                self.player_1 = message;
            }
            Message::Fighter1(message) => {
                self.fighter_1 = message;
            }
            Message::Player2(message) => {
                self.player_2 = message;
            }
            Message::Fighter2(message) => {
                self.fighter_2 = message;
            }
            Message::StartTime(message) => {
                self.start_time = message;
            }
            Message::EndTime(message) => {
                self.end_time = message;
            }
            Message::Submit => {
                let data = self.clone();
                self.message = String::from("Working on it...");
                return Task::future(async move {
                    let msg = task::spawn_blocking(move || {
                        let mut msg = String::from("Finished");
                        if data.generate_thumbnail {
                            let filename_png =
                                PathBuf::from(&data.output_folder).join(get_filename(
                                    &data.tournament_name,
                                    &data.round_name,
                                    &data.player_1,
                                    &data.player_2,
                                    "jpg",
                                ));
                            write_thumbnail(
                                filename_png,
                                &data.tournament_name,
                                &data.round_name,
                                &data.date,
                                &data.player_1,
                                &data.fighter_1,
                                &data.player_2,
                                &data.fighter_2,
                            );
                            msg.push_str(" generating thumbnail");
                            if data.generate_video {
                                msg.push_str(" and");
                            }
                        }

                        if data.generate_video {
                            let filename_mp4 =
                                PathBuf::from(&data.output_folder).join(get_filename(
                                    &data.tournament_name,
                                    &data.round_name,
                                    &data.player_1,
                                    &data.player_2,
                                    "mp4",
                                ));
                            trim_video(
                                "static/ffmpeg.exe",
                                &data.input_file,
                                filename_mp4.to_str().expect("Invalid filename"),
                                &data.start_time,
                                &data.end_time,
                            );
                            msg.push_str(" generating video");
                        }
                        msg.push('!');
                        msg
                    })
                    .await
                    .unwrap();
                    Message::UpdateMsg(msg)
                });
            }
            Message::UpdateMsg(message) => {
                self.message = message;
                return Task::done(Message::End);
            }
            Message::GenerateThumbnail(message) => self.generate_thumbnail = message,
            Message::GenerateVideo(message) => self.generate_video = message,
            Message::ReloadConfig => {
                crate::img::reload_config();
            }
        }
        Task::done(Message::UpdateMsg(String::new()))
    }

    fn view(&self) -> Element<Message> {
        let char_imgs = CHAR_IMGS.read().expect("Poisoned!").clone();
        Column::new()
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(10.0))
                    .push(
                        Text::new("Input File:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.input_file)
                            .width(iced::Length::FillPortion(5))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::InputFile),
                    )
                    .push(
                        Button::new(Text::new("Browse..."))
                            .width(100.0)
                            .on_press(Message::BrowseFile),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(10.0))
                    .push(
                        Text::new("Output Folder:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.output_folder)
                            .width(iced::Length::FillPortion(5))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::OutputFolder),
                    )
                    .push(
                        Button::new(Text::new("Browse..."))
                            .width(100.0)
                            .on_press(Message::BrowserFolder),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Text::new("Tournament Name:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.tournament_name)
                            .width(iced::Length::FillPortion(5))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::TournamentName),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Text::new("Round Name:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.round_name)
                            .width(iced::Length::FillPortion(5))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::RoundName),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Text::new("Date:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.date)
                            .width(iced::Length::FillPortion(5))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::Date),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Text::new("Player 1:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.player_1)
                            .width(iced::Length::FillPortion(4))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::Player1),
                    )
                    .push(
                        pick_list(char_imgs.clone(), Some(&self.fighter_1), Message::Fighter1)
                            .width(iced::Length::FillPortion(1)),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Text::new("Player 2:")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.player_2)
                            .width(iced::Length::FillPortion(4))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::Player2),
                    )
                    .push(
                        pick_list(char_imgs.clone(), Some(&self.fighter_2), Message::Fighter2)
                            .width(iced::Length::FillPortion(1)),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(0.0))
                    .push(
                        Text::new("Starting and Ending Time (HH:MM:SS):")
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left),
                    )
                    .push(
                        TextInput::new("", &self.start_time)
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::StartTime),
                    )
                    .push(
                        TextInput::new("", &self.end_time)
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Left)
                            .on_input(Message::EndTime),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(10.0))
                    .push(
                        Checkbox::new("Generate Thumbnail", self.generate_thumbnail)
                            .on_toggle(Message::GenerateThumbnail)
                            .width(iced::Length::FillPortion(1)),
                    )
                    .push(
                        Checkbox::new("Generate Video", self.generate_video)
                            .on_toggle(Message::GenerateVideo)
                            .width(iced::Length::FillPortion(1)),
                    ),
            )
            .push(
                Row::new()
                    .spacing(5)
                    .align_y(iced::alignment::Vertical::Center)
                    .padding(iced::Padding::new(10.0).top(10.0))
                    .push(
                        Button::new(Text::new("Submit"))
                            .width(100.0)
                            .on_press(Message::Submit),
                    )
                    .push(
                        widget::text!("{}", self.message)
                            .width(iced::Length::FillPortion(1))
                            .align_x(iced::alignment::Horizontal::Center),
                    )
                    .push(
                        Button::new(Text::new("Reload Config"))
                            .width(200.0)
                            .on_press(Message::ReloadConfig),
                    ),
            )
            .into()
    }
}
