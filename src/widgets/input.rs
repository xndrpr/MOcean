use std::time::Duration;

use piston::Key;

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

    async fn insert_char(&self, input_string: &str, pos: usize, ch: char) -> String {
        let mut char_vec: Vec<char> = input_string.chars().collect();
        char_vec.insert(pos, ch);
        char_vec.into_iter().collect()
    }

    pub async fn press(&mut self, key: Key, width: f64) {
        let mut key = format!("{:?}", key)
            .replace("Minus", "-")
            .replace("Space", " ")
            .replace("Slash", "/");

        if self.case == Case::UPPER {
            key = key.replace("Equals", "+");
            key = key.replace("D8", "*");
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
            key = key.replace("NumPad", "");
            key = key.replace("NumPadDivide", "/");
        }

        if key.to_lowercase().contains("shift") {
            self.case = Case::UPPER;

            return;
        }

        if key.to_lowercase().contains("backspace") && !self.text.is_empty() {
            self.text.pop();

            if self.text.len() <= 0 {
                self.text = "\n".to_string();
            }

            return;
        }

        if key.len() > 2 {
            return;
        }

        if self.case == Case::LOWER {
            self.text = self
                .insert_char(
                    &self.text,
                    self.text.len(),
                    key.to_lowercase().chars().next().unwrap_or_default(),
                )
                .await;
        } else {
            self.text = self
                .insert_char(&self.text, self.text.len(), key.chars().next().unwrap())
                .await;
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
