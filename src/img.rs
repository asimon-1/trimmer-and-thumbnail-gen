use ab_glyph::{FontRef, PxScale};
use cached::proc_macro::cached;
use image::buffer::ConvertBuffer;
use image::imageops::overlay;
use image::{open, ImageBuffer, Rgb, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use imageproc::geometric_transformations::{rotate, Interpolation};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, RwLock};

#[derive(Deserialize, Clone)]
pub struct Config {
    width: u32,
    height: u32,
    base_path: String,
    char_img_path: String,
    font: String,
    background_images: Vec<String>,
    foreground_images: Vec<String>,
    positioned_texts: Vec<PositionedText>,
}

#[derive(Deserialize, Clone)]
pub struct PositionedText {
    text: String,
    x: i32,
    y: i32,
    scale: f32,
    theta: f32,
}

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let config = load_config_from_file();
    RwLock::new(config)
});

pub static CHAR_IMGS: LazyLock<RwLock<Vec<String>>> =
    LazyLock::new(|| RwLock::new(load_image_filenames()));

static FONT_BYTES: LazyLock<RwLock<Vec<u8>>> = LazyLock::new(|| {
    let config = get_config();
    RwLock::new(
        fs::read(Path::new(&config.base_path).join(&config.font)).expect("Could not load font!"),
    )
});

fn load_config_from_file() -> Config {
    let path = Path::new("static/config.json");
    let data = fs::read_to_string(path).expect("Failed to read config.json");
    serde_json::from_str(&data).expect("Failed to parse config.json")
}

fn load_image_filenames() -> Vec<String> {
    let config = get_config();
    fs::read_dir(Path::new(&config.base_path).join(&config.char_img_path))
        .expect("Could not open char_img_path")
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .map(|f| f.file_name().to_str().unwrap().to_string())
        .collect()
}

fn get_config() -> Config {
    CONFIG
        .read()
        .expect("RwLock poisoned during get_config()")
        .clone()
}

pub fn reload_config() {
    let new_config = load_config_from_file();
    let mut conf_write_guard = CONFIG
        .write()
        .expect("RwLock poisoned during reload_config()");
    let mut font_write_guard = FONT_BYTES
        .write()
        .expect("Font RwLock poisoned during reload_config()");
    *font_write_guard = fs::read(Path::new(&new_config.base_path).join(&new_config.font))
        .expect("Could not load font!");
    *conf_write_guard = new_config;
    println!("Configuration reloaded.");
}

pub fn get_filename(
    tournament_name: &str,
    round_name: &str,
    player_1: &str,
    player_2: &str,
    extension: &str,
) -> PathBuf {
    PathBuf::from(if round_name != "" {
        format!("{tournament_name} - {round_name} - {player_1} vs {player_2}.{extension}")
    } else {
        format!("{tournament_name} - {player_1} vs {player_2}.{extension}")
    })
}

pub fn write_thumbnail(
    filename: impl AsRef<Path>,
    tournament_name: &str,
    round_name: &str,
    date: &str,
    player_1: &str,
    fighter_1: &str,
    player_2: &str,
    fighter_2: &str,
) {
    let config = get_config();
    let mut base_img = RgbaImage::new(config.width, config.height);
    let mut layers = Vec::new();
    config
        .background_images
        .iter()
        .for_each(|filename| layers.push(load_image(filename)));
    layers.push(load_image(
        Path::new(&config.char_img_path)
            .join(fighter_1)
            .to_str()
            .expect("Could not build fighter_1 filepath"),
    ));
    layers.push(load_image(
        Path::new(&config.char_img_path)
            .join(fighter_2)
            .to_str()
            .expect("Could not build fighter_1 filepath"),
    ));
    config
        .foreground_images
        .iter()
        .for_each(|filename| layers.push(load_image(filename)));
    config.positioned_texts.iter().for_each(|positioned_text| {
        let to_draw = match positioned_text.text.as_ref() {
            "TOURNAMENT_NAME" => PositionedText {
                text: tournament_name.to_string(),
                ..*positioned_text
            },
            "PLAYER_1" => PositionedText {
                text: player_1.to_string(),
                ..*positioned_text
            },
            "PLAYER_2" => PositionedText {
                text: player_2.to_string(),
                ..*positioned_text
            },
            "ROUND_NAME" => PositionedText {
                text: round_name.to_string(),
                ..*positioned_text
            },
            "DATE" => PositionedText {
                text: date.to_string(),
                ..*positioned_text
            },
            _ => positioned_text.clone(),
        };
        layers.push(draw_centered_text(
            config.width,
            config.height,
            &to_draw.text,
            to_draw.x,
            to_draw.y,
            to_draw.scale,
            to_draw.theta,
        ))
    });
    layers
        .iter()
        .for_each(|layer| overlay(&mut base_img, layer, 0, 0));
    let base_img = rgba8_to_rgb8(base_img);
    base_img.save(filename).expect("Could not save the image");
}

#[cached(
    key = "(String, i32, i32, i32, i32)",
    convert = r#"{ (String::from(text), x_px, y_px, (scale * 1000.0) as i32, (rotation * 1000.0) as i32) }"#
)]
fn draw_centered_text(
    width: u32,
    height: u32,
    text: &str,
    x_px: i32,
    y_px: i32,
    scale: f32,
    rotation: f32,
) -> ImageBuffer<Rgba<u8>, Vec<<Rgba<u8> as image::Pixel>::Subpixel>> {
    let mut img = RgbaImage::new(width, height);
    let binding = FONT_BYTES.read().expect("FONT_BYTES poisoned");
    let font = FontRef::try_from_slice(&binding).expect("Could not load the font!");
    let pxscale = PxScale::from(scale);
    let size = text_size(pxscale, &font, text);
    let color = Rgba([227, 228, 229, 255]);
    draw_text_mut(
        &mut img,
        color,
        x_px - (size.0 as i32 / 2),
        y_px - (size.1 as i32 / 2),
        pxscale,
        &font,
        text,
    );
    rotate(
        &img,
        (x_px as f32, y_px as f32),
        rotation,
        Interpolation::Bicubic,
        Rgba([0, 0, 0, 0]),
    )
}

fn rgba8_to_rgb8(input: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    input.convert()
}

#[cached(key = "String", convert = r#"{ String::from(filename) }"#)]
fn load_image(filename: &str) -> RgbaImage {
    open(Path::new(&get_config().base_path).join(filename))
        .expect(&format!("Couldn't open {filename}"))
        .to_rgba8()
}
