extern crate csscolorparser;

use std::str::FromStr;

use csscolorparser::Color;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum StrOrNum {
    Str(String),
    Num(i32),
}

pub type Length = StrOrNum;
#[derive(Deserialize, Debug)]
pub struct SideLengths {
    top: Option<Length>,
    bottom: Option<Length>,
    left: Option<Length>,
    right: Option<Length>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum LengthOrSideLengths {
    Length(Length),
    SideLengths(SideLengths),
}

#[derive(Debug)]
pub enum ColorOrNone {
    None,
    Color(Color),
}

impl<'de> Deserialize<'de> for ColorOrNone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        match string.as_str() {
            "none" => Ok(ColorOrNone::None),
            default => Color::from_str(default)
                .map(|color| ColorOrNone::Color(color))
                .map_err(serde::de::Error::custom),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    x: Option<Length>,
    y: Option<Length>,
}

// Layout
pub type Offset = Coordinate;
pub type Margin = LengthOrSideLengths;
pub type Padding = LengthOrSideLengths;

#[derive(Deserialize, Debug)]
pub struct Layout {
    width: Option<Length>,
    height: Option<Length>,
    offset: Option<Offset>,
    margin: Option<Margin>,
    padding: Option<Padding>,
}

// Style
#[derive(Deserialize, Debug)]
pub struct Style {
    foreground: Option<ColorOrNone>,
    background: Option<ColorOrNone>,
    underline: Option<ColorOrNone>,
    overline: Option<ColorOrNone>,
    strikethrough: Option<ColorOrNone>,
    border: Option<BorderStyle>,
    line: Option<LineStyle>,
    #[serde(rename = "letter-spacing")]
    letter_spacing: Option<String>,
    cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Font {
    family: Option<String>,
    style: Option<String>,
    weight: Option<StrOrNum>,
    letter_spacing: Option<Length>,
}

#[derive(Deserialize, Debug)]
pub struct BorderStyle {
    color: Option<Color>,
    size: Option<Length>,
    radius: Option<Length>,
}

#[derive(Deserialize, Debug)]
pub struct LineStyle {
    color: Option<Color>,
    size: Option<Length>,
    height: Option<Length>,
}

// Events
#[derive(Deserialize, Debug)]
pub struct Events<T = String> {
    click: Option<ClickEvents<T>>,
    scroll: Option<ScrollEvents<T>>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateEvents {
    interval: Option<i32>,
    on: Events<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ClickEvents<T> {
    left: Option<T>,
    middle: Option<T>,
    right: Option<T>,
    thumb1: Option<T>,
    thumb2: Option<T>,
}

#[derive(Deserialize, Debug)]
pub struct ScrollEvents<T> {
    down: Option<T>,
    up: Option<T>,
}
