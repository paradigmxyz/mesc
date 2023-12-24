use crate::MescCliError;
use mesc::MescError;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use toolstr::{Color, FontStyle};

const DEFAULT_COLOR_TITLE: Color = toolstr::Color::TrueColor { r: 206, g: 147, b: 249 };
const DEFAULT_COLOR_METAVAR: Color = toolstr::Color::TrueColor { r: 137, g: 233, b: 253 };
const DEFAULT_COLOR_DESCRIPTION: Color = toolstr::Color::TrueColor { r: 185, g: 242, b: 159 };
const DEFAULT_COLOR_OPTION: Color = toolstr::Color::TrueColor { r: 100, g: 170, b: 170 };
const DEFAULT_COLOR_CONTENT: Color = toolstr::Color::TrueColor { r: 241, g: 250, b: 140 };
const DEFAULT_COLOR_COMMENT: Color = toolstr::Color::TrueColor { r: 98, g: 114, b: 164 };

pub(crate) fn get_theme_font_style(id: &str) -> Result<FontStyle, MescCliError> {
    match get_cli_theme() {
        Ok(theme) => match theme.get(id) {
            Some(style) => Ok(style.clone()),
            None => Ok(FontStyle::default()),
        },
        _ => match id {
            "title" => Ok(DEFAULT_COLOR_TITLE.into()),
            "metavar" => Ok(DEFAULT_COLOR_METAVAR.into()),
            "description" => Ok(DEFAULT_COLOR_DESCRIPTION.into()),
            "option" => Ok(DEFAULT_COLOR_OPTION.into()),
            "content" => Ok(DEFAULT_COLOR_CONTENT.into()),
            "comment" => Ok(DEFAULT_COLOR_COMMENT.into()),
            _ => Ok(FontStyle::default()),
        },
    }
}

fn get_cli_theme() -> Result<HashMap<String, FontStyle>, MescCliError> {
    let global_metadata = mesc::get_global_metadata()?;
    let path: Vec<&str> = vec![];
    let cli_theme: HashMap<String, String> = match global_metadata.get("cli_theme") {
        Some(value) => get_value_at(value, &path)?,
        None => return Err(MescError::IntegrityError("no cli theme".to_string()).into()),
    };

    let cli_theme = cli_theme
        .into_iter()
        .map(|(k, v)| {
            let style: FontStyle = toolstr::hex_to_color(&v).map_err(MescCliError::from)?.into();
            Ok((k, style))
        })
        .collect::<Result<HashMap<String, FontStyle>, MescCliError>>()?;

    Ok(cli_theme)
}

fn get_value_at<T>(root: &serde_json::Value, path: &[&str]) -> Result<T, MescCliError>
where
    T: DeserializeOwned,
{
    let mut current = root;

    for key in path {
        current = match current.get(key) {
            Some(value) => value,
            None => return Err(MescError::IntegrityError("missing path".to_string()).into()),
        };
    }

    Ok(serde_json::from_value(current.clone())?)
}
