# Video Trimmer and Thumbnail Generator

A desktop application to help with splitting tournament VODs into individual sets with associated thumbnails.

## Overview

**Trimmer and Thumbnail Gen** is a desktop tool designed for TOs, streamers, and video editors who work with tournament VODs. This app streamlines the process of:
- Splitting long VODs into individual set videos
- Generating and assigning custom thumbnails for each set

## Features

- Efficient trimming of a long video to the start/end time of a set
- Simultaneous thumbnail creation

## Getting Started

### Building the Application

1. **Clone the repository:**
   ```sh
   git clone https://github.com/asimon-1/trimmer-and-thumbnail-gen.git
   cd trimmer-and-thumbnail-gen
   ```

2. **Build the executable:**
   ```sh
   cargo build --release
   ```

   The compiled binary will be located at `target/release/`.

3. **Prepare static resources:**
   - Ensure the `static` directory exists at the project root.
   - Place all necessary resource files (e.g., default images, templates, or fonts) into the `static` directory.
   - The application expects these resources at runtime.

### Running the Application

```sh
./target/release/trimmer-and-thumbnail-gen
```

## Shipping the Application

To distribute the app:

1. Copy the built executable (`trimmer-and-thumbnail-gen` or `trimmer-and-thumbnail-gen.exe`).
2. Include the entire `static` directory alongside the executable.
3. Provide a sample or default `config.json` file (see below for documentation).
4. Package together (e.g., in a `.zip` or installer) for your target platform.

**Directory structure example:**

```
trimmer-and-thumbnail-gen/
│
├── trimmer-and-thumbnail-gen.exe
└── static/
    ├── config.json
    ├── ffmpeg.exe
    ├── overlay_template.png
    └── characters/
        │
        ├── mario.png
        ├── ...
        └── bowser.png
```

## Configuration: `config.json`

The application uses a `config.json` file found in the `static` directory to control thumbnail generation, layout, and appearance.

### Example `config.json`

```json
{
    "width": 1920,
    "height": 1080,
    "base_path": "static",
    "char_img_path": "characters",
    "font": "LEMONMILK-Bold.otf",
    "background_images": [
        "Thumbnail_Background.png"
    ],
    "foreground_images": [
        "Player_Banner.png",
        "Tournament_Banner.png"
    ],
    "positioned_texts": [
        {
            "text": "TOURNAMENT_NAME",
            "x": 960,
            "y": 70,
            "scale": 100.0,
            "theta": 0.0
        },
        {
            "text": "PLAYER_1",
            "x": 400,
            "y": 822,
            "scale": 100.0,
            "theta": 0.100
        },
        {
            "text": "PLAYER_2",
            "x": 1520,
            "y": 822,
            "scale": 100.0,
            "theta": 6.183
        },
        {
            "text": "ROUND_NAME",
            "x": 268,
            "y": 950,
            "scale": 60.0,
            "theta": 0.090
        },
        {
            "text": "DATE",
            "x": 1652,
            "y": 950,
            "scale": 60.0,
            "theta": 6.193
        },
        {
            "text": "Portrait Art by @ElevenZM",
            "x": 1770,
            "y": 1050,
            "scale": 20.0,
            "theta": 0.0
        }
    ]
}
```

### Field Reference

| Field                | Type      | Description                                                                                           |
|----------------------|-----------|-------------------------------------------------------------------------------------------------------|
| `width`              | integer   | Width of the output thumbnail image in pixels.                                                        |
| `height`             | integer   | Height of the output thumbnail image in pixels.                                                       |
| `base_path`          | string    | Base directory for static resources referenced in this config (e.g., images, fonts).                  |
| `char_img_path`      | string    | Subdirectory under `base_path` where character images are stored.                                     |
| `font`               | string    | Font file (relative to `base_path`) to use for all text rendering.                                    |
| `background_images`  | array     | List of background image filenames (relative to `base_path`) to be layered at the bottom.             |
| `foreground_images`  | array     | List of foreground image filenames (relative to `base_path`) to be layered above text and characters. |
| `positioned_texts`   | array     | List of text objects specifying what text to render, where, and how. See below for details.           |

#### `positioned_texts` Objects

Each object in the `positioned_texts` array has:

| Field    | Type    | Description                                                                                                                                                                                        |
|----------|---------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `text`   | string  | The actual text to be displayed in the image. The specific strings `TOURNAMENT_NAME`, `PLAYER_1`, `PLAYER_2`, `ROUND_NAME`, and `DATE` are replaced by the user's entries in the GUI at runtime.   |
| `x`      | number  | X position (in pixels) of the text anchor point on the thumbnail.                                                                                                                                  |
| `y`      | number  | Y position (in pixels) of the text anchor point on the thumbnail.                                                                                                                                  |
| `scale`  | number  | Size of the text.                                                                                                                                                                                  |
| `theta`  | number  | Rotation of the text, in radians.                                                                                                                                                                  |

You can add as many positioned text objects as needed to customize what appears on the thumbnails and where.

**Note:**  
- `background_images` are rendered first, then character images, then positioned texts, and finally `foreground_images`.
- The character images are not repositioned during the image layering. For a "Fighter X versus Fighter Y" style thumbnail, it is recommended to have one image representing the character on the left side and a separate image where the character is positioned on the right side.

## Example Usage

1. Prepare your `config.json` with the desired styling of the thumbnail.
2. Place any required resources in the `static` directory.
3. Place any character images in the `static/characters` directory. These should be the same size as the width and height properties in the config.json
4. Run the application. 
   1. Select the original VOD as the input file, and select the output directory.
   2. Fill out the remaining fields with the details of the tournament and the individual set.
   3. The fighter dropdowns are only used if generating a thumbnail
   4. The starting and ending timestamps are only used if generating a video
   5. Hit submit

To fine-tune the thumbnail image styling, you can iteratively adjust the `config.json` file, pressing the "Reload Config" button and re-submitting the thumbnail generation task.

## Contributing

Pull requests are welcome!

## License

This project is licensed under the MIT License.

Feel free to use or adapt it as you see fit.

## Acknowledgements

- Uses [ffmpeg](https://ffmpeg.org/) for video processing
- For the Super Smash Brothers character art provided in `static/characters`, the original art is created by Yusuke Nakano. ElevenZM isolated each character.
