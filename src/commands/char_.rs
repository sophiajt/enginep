use crate::{CommandArgs, Example, Scope, ValueIterator, WholeStreamCommand};
use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value};
use nu_source::{Tag, Tagged};

pub struct Char;

struct CharArgs {
    name: Tagged<String>,
    rest: Vec<Tagged<String>>,
    unicode: bool,
}

impl WholeStreamCommand for Char {
    fn name(&self) -> &str {
        "char"
    }

    fn signature(&self) -> Signature {
        Signature::build("char")
            .required(
                "character",
                SyntaxShape::Any,
                "the name of the character to output",
            )
            .rest(SyntaxShape::String, "multiple unicode bytes")
            .switch("unicode", "unicode string i.e. 1f378", Some('u'))
    }

    fn usage(&self) -> &str {
        "Output special characters (eg. 'newline')."
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Output newline",
                example: r#"char newline"#,
                result: Some(vec![Value::from("\n")]),
            },
            Example {
                description: "Output prompt character, newline and a hamburger character",
                example: r#"echo $(char prompt) $(char newline) $(char hamburger)"#,
                result: Some(vec![
                    UntaggedValue::string("\u{25b6}").into(),
                    UntaggedValue::string("\n").into(),
                    UntaggedValue::string("\u{2261}").into(),
                ]),
            },
            Example {
                description: "Output unicode character",
                example: r#"char -u 1f378"#,
                result: Some(vec![Value::from("\u{1f378}")]),
            },
            Example {
                description: "Output multi-byte unicode character",
                example: r#"char -u 1F468 200D 1F466 200D 1F466"#,
                result: Some(vec![Value::from(
                    "\u{1F468}\u{200D}\u{1F466}\u{200D}\u{1F466}",
                )]),
            },
        ]
    }

    fn run(&self, args: CommandArgs, scope: &Scope) -> Result<ValueIterator, ShellError> {
        Ok(Box::new(std::iter::once(
            UntaggedValue::string("\n").into_untagged_value(),
        )))
    }
}

fn string_to_unicode_char(s: &str, t: &Tag) -> Result<char, ShellError> {
    let decoded_char = u32::from_str_radix(s, 16)
        .ok()
        .and_then(std::char::from_u32);

    if let Some(ch) = decoded_char {
        Ok(ch)
    } else {
        Err(ShellError::labeled_error(
            "error decoding unicode character",
            "error decoding unicode character",
            t,
        ))
    }
}

fn str_to_character(s: &str) -> Option<String> {
    match s {
        "newline" | "enter" | "nl" => Some("\n".into()),
        "tab" => Some("\t".into()),
        "sp" | "space" => Some(" ".into()),
        // Unicode names came from https://www.compart.com/en/unicode
        // Private Use Area (U+E000-U+F8FF)
        // Unicode can't be mixed with Ansi or it will break width calculation
        "branch" => Some('\u{e0a0}'.to_string()),  // 
        "segment" => Some('\u{e0b0}'.to_string()), // 

        "identical_to" | "hamburger" => Some('\u{2261}'.to_string()), // ≡
        "not_identical_to" | "branch_untracked" => Some('\u{2262}'.to_string()), // ≢
        "strictly_equivalent_to" | "branch_identical" => Some('\u{2263}'.to_string()), // ≣

        "upwards_arrow" | "branch_ahead" => Some('\u{2191}'.to_string()), // ↑
        "downwards_arrow" | "branch_behind" => Some('\u{2193}'.to_string()), // ↓
        "up_down_arrow" | "branch_ahead_behind" => Some('\u{2195}'.to_string()), // ↕

        "black_right_pointing_triangle" | "prompt" => Some('\u{25b6}'.to_string()), // ▶
        "vector_or_cross_product" | "failed" => Some('\u{2a2f}'.to_string()),       // ⨯
        "high_voltage_sign" | "elevated" => Some('\u{26a1}'.to_string()),           // ⚡
        "tilde" | "twiddle" | "squiggly" | "home" => Some("~".into()),              // ~
        "hash" | "hashtag" | "pound_sign" | "sharp" | "root" => Some("#".into()),   // #

        // Weather symbols
        "sun" | "sunny" | "sunrise" => Some("☀️".to_string()),
        "moon" => Some("🌛".to_string()),
        "cloudy" | "cloud" | "clouds" => Some("☁️".to_string()),
        "rainy" | "rain" => Some("🌦️".to_string()),
        "foggy" | "fog" => Some("🌫️".to_string()),
        "mist" | "haze" => Some("\u{2591}".to_string()),
        "snowy" | "snow" => Some("❄️".to_string()),
        "thunderstorm" | "thunder" => Some("🌩️".to_string()),

        "bel" => Some('\x07'.to_string()),       // Terminal Bell
        "backspace" => Some('\x08'.to_string()), // Backspace

        _ => None,
    }
}
