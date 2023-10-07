use std::time::Duration;

use piston::Key;
use regex::Regex;

use crate::cursor::Cursor;

#[derive(PartialEq, Debug)]
pub enum Case {
    LOWER,
    UPPER,
}

pub struct Input {
    pub text: String,
    pub backspace: bool,
    pub cursor: Cursor,
    case: Case,
}

impl Input {
    pub async fn new() -> Self {
        Self {
            text: "\n".to_string(),
            case: Case::LOWER,
            backspace: false,
            cursor: Cursor::new(0.0, 0.0, Duration::from_millis(350)),
        }
    }

    async fn power(&self, force: bool) -> String {
        let re;
        if force {
            re = Regex::new(r"(\w+)\^(-?\d+)([+\-×/_=! ]*)").unwrap();
        } else {
            re = Regex::new(r"(\w+|\d+)\^(-?\d+)([+\-×/_=!])").unwrap();
        }

        let replaced_string = re
            .replace_all(&self.text, |caps: &regex::Captures| {
                let base = &caps[1];
                let num = &caps[2];
                let symbol = &caps[3];

                let superscripted_num = num
                    .chars()
                    .map(|c| match c {
                        '1' => '¹',
                        '2' => '²',
                        '3' => '³',
                        '4' => '⁴',
                        '5' => '⁵',
                        '6' => '⁶',
                        '7' => '⁷',
                        '8' => '⁸',
                        '9' => '⁹',
                        '0' => '⁰',
                        '-' => '⁻',
                        _ => c,
                    })
                    .collect::<String>();

                format!("{}{}{}", base, superscripted_num, symbol)
            })
            .to_string();

        replaced_string
    }

    pub async fn press(&mut self, key: Key, width: f64) {
        let mut key = format!("{:?}", key)
            .replace("Minus", "-")
            .replace("Space", " ");

        if self.case == Case::UPPER {
            key = key.replace("Equals", "+");
            key = key.replace("D8", "×");
            key = key.replace("D6", "^");
            key = key.replace("D0", ")");
            key = key.replace("D9", "(");
        } else {
            key = key.replace("Equals", "=");
            key = key.replace("D8", "8");
            key = key.replace("D6", "6");
        }

        if self.text.chars().filter(|&c| c == '\n').count() < (width / 100.0) as usize {
            key = key.replace("Return", "\n\n");
        }

        if key.len() > 1 {
            key = key.replace("D", "");
        }

        if key.to_lowercase().contains("tab") {
            self.text = self.power(true).await;
        } else if self.text.contains("^") {
            self.text = self.power(false).await;
        }

        if key.to_lowercase().contains("shift") {
            self.case = Case::UPPER;

            return;
        }

        if key.to_lowercase().contains("backspace") && !self.text.is_empty() {
            if self.case == Case::LOWER {
                self.text.pop();
                if self.text.len() <= 0 {
                    self.text = "\n".to_string();
                }
            } else {
                self.text = self.remove_last_word(self.text.clone()).await;
                if self.text.len() <= 0 {
                    self.text = "\n".to_string();
                }
            }

            return;
        }

        if key.len() > 2 {
            return;
        }

        if self.case == Case::LOWER {
            self.text = format!("{}{}", self.text, key.to_lowercase());
        } else {
            self.text = format!("{}{}", self.text, key);
        }
    }

    async fn remove_last_word(&self, input: String) -> String {
        if input.contains(' ') {
            if let Some(last_space_idx) = input.rfind(' ') {
                let result = input[..last_space_idx].trim().to_string();
                return result;
            }
        } else if input.contains("\n") {
            if let Some(last_space_idx) = input.rfind('\n') {
                let result = input[..last_space_idx].trim().to_string();
                return result;
            }
        }
        "\n".to_string()
    }

    pub async fn release(&mut self, key: Key) {
        let key = format!("{:?}", key);

        if key.to_lowercase().contains("shift") {
            self.case = Case::LOWER;
            return;
        }
    }
}
