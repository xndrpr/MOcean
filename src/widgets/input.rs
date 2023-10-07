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

    pub async fn press(&mut self, key: Key) {
        let mut key = format!("{:?}", key)
            .replace("Minus", "-")
            .replace("Equals", "=")
            .replace("Plus", "+")
            .replace("Space", " ");

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

        if key.len() > 1 {
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
