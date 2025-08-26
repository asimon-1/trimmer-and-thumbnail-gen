use std::io::{self, Write};
use std::process::Command;

pub fn trim_video(
    ffmpeg_path: impl AsRef<std::ffi::OsStr>,
    input_path: &str,
    output_path: &str,
    start_time: &str,
    end_time: &str,
) {
    let output = Command::new(ffmpeg_path)
        .args([
            "-ss",
            start_time,
            "-to",
            end_time,
            "-i",
            input_path,
            "-c",
            "copy",
            "-y",
            output_path,
        ])
        .output()
        .expect("ffmpeg call failed");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
