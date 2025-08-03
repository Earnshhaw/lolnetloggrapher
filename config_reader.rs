use anyhow::Result;
use config as config_rs;
use plotters::style::RGBColor;
use rand::Rng;
use serde::Deserialize;
use serde::Deserializer;
use std::path::Path;

/// Custom deserializer for RGBColor
fn deserialize_rgb<'de, D>(deserializer: D) -> std::result::Result<RGBColor, D::Error>
where
    D: Deserializer<'de>,
{
    let arr: [u8; 3] = Deserialize::deserialize(deserializer)?;
    Ok(RGBColor(arr[0], arr[1], arr[2]))
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub filename: String,

    #[serde(deserialize_with = "deserialize_rgb")]
    pub captioncolor: RGBColor,

    pub captionfontfamily: String,

    #[serde(rename = "captionsize")]
    pub captionfontsize: i32,

    #[serde(deserialize_with = "deserialize_rgb")]
    pub linecolor: RGBColor,
}

pub fn get_cfg() -> Result<Config> {
    let path = Path::new("config.toml");

    if path.exists() {
        let config = config_rs::Config::builder()
            .add_source(config_rs::File::with_name("config.toml")) // This looks for "config.toml"
            .build()?
            .try_deserialize::<Config>()?;
        Ok(config)
    } else {
        let default = Config {
            width: 640,
            height: 480,
            filename: get_name()?.to_string(),
            captionfontfamily: "serif".to_string(),
            captionfontsize: 30,
            captioncolor: RGBColor(255, 255, 255),
            linecolor: RGBColor(255, 0, 0),
        };
        Ok(default)
    }
}

pub fn get_name() -> Result<String> {
    let file_path = Path::new("Graph.png");
    let mut title = String::from("Graph");

    if file_path.exists() {
        let mut rng = rand::rng();
        let rndint: u16 = rng.random_range(0..1000);
        title.push_str(&rndint.to_string());
    }

    title.push_str(".png");
    Ok(title)
}
