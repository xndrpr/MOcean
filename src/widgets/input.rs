use piston::{Button, Key};
use piston_window::PistonWindow;

#[derive(PartialEq, Debug)]
pub enum Case {
    LOWER,
    UPPER,
}

pub struct Input {
    pub text: String,
    pub backspace: bool,
    case: Case,
}

impl Input {
    pub async fn new() -> Self {
        Self {
            text: "".to_string(),
            case: Case::LOWER,
            backspace: false,
        }
    }

    pub async fn press(&mut self, key: Key, width: f64) {
        let mut key = format!("{:?}", key)
            .replace("Minus", "-")
            .replace("Space", " ");

        if self.case == Case::UPPER {
            key = key.replace("Equals", "+");
            key = key.replace("D8", "×");
        } else {
            key = key.replace("Equals", "=");
            key = key.replace("D8", "8");
        }

        if self.text.chars().filter(|&c| c == '\n').count() < (width / 100.0) as usize {
            key = key.replace("Return", "\n");
        }

        if key.len() > 1 {
            key = key.replace("D", "");
        }

        if key.to_lowercase().contains("shift") {
            self.case = Case::UPPER;

            return;
        }

        if key.to_lowercase().contains("backspace") && !self.text.is_empty() {
            println!("{:?}", self.case);

            if self.case == Case::LOWER {
                self.text.pop();
            } else {
                self.text = self.remove_last_word(self.text.clone()).await;
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
        String::new()
    }

    pub async fn release(&mut self, key: Key) {
        let key = format!("{:?}", key);

        if key.to_lowercase().contains("shift") {
            self.case = Case::LOWER;
            return;
        }
    }
}
