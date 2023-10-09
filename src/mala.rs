use regex::Regex;
use std::fmt;

pub struct MaLa {
    pub content: String,
}

impl MaLa {
    pub fn new() -> MaLa {
        MaLa {
            content: "".to_string(),
        }
    }

    async fn power(&mut self) {
        let re =
            Regex::new(r"\((?:[^()]+|\([^()]*\))\)\s*\^\s*-?\d+|\b(?:[a-zA-Z]+|\d+)\s*\^\s*-?\d+")
                .unwrap();

        while let Some(mat) = re.captures(&self.content) {
            let base = mat[0].split("^").nth(0).unwrap();
            let exponent = mat[0].split("^").nth(1).unwrap();
            let result = format!("$({{{}}}^{{{}}})", base, exponent)
                .trim()
                .replace("\n", "");

            self.content.replace_range(
                mat.get(0).unwrap().start()..mat.get(0).unwrap().end(),
                &result,
            );
        }
    }

    async fn fraction(&mut self) {
        let re = Regex::new(
            r"([0-9]*[a-zA-Z]*)(?:\s*\^\s*[0-9]+)?\s*\/\s*(\d+)|\(((?:[^\(\)]+|\([^)]+\))*)\)\s*\/\s*(\d+)",
        )
        .unwrap();

        while let Some(mat) = re.captures(&self.content) {
            let numerator = mat[0].split("/").nth(0).unwrap();
            let denominator = mat[0].split("/").nth(1).unwrap();
            let result = format!("$({{{}}}/{{{}}})", numerator, denominator);
            self.content.replace_range(
                mat.get(0).unwrap().start()..mat.get(0).unwrap().end(),
                &result,
            );
        }
    }

    pub async fn format(&mut self, plain_text: &str) {
        self.content = plain_text.to_string();
        self.fraction().await;
        self.power().await;
    }
}

impl fmt::Display for MaLa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn frac_n_power() {
        let mut mala = MaLa::new();
        mala.format("x^2 + 2/3").await;
        assert_eq!(mala.content, "$({x}^{2}) + $({2}/{3})");
    }

    #[tokio::test]
    async fn negative_power() {
        let mut mala = MaLa::new();
        mala.format("x^-2").await;
        assert_eq!(mala.content, "$({x}^{-2})");
    }

    #[tokio::test]
    async fn negative_n_positive_power() {
        let mut mala = MaLa::new();
        mala.format("x^-22 + x^2").await;
        println!("{}", mala.content);
        assert_eq!(mala.content, "$({x}^{-22}) + $({x}^{2})");
    }

    #[tokio::test]
    async fn power_n_power() {
        let mut mala = MaLa::new();
        mala.format("x^2 + 2^2").await;
        assert_eq!(mala.content, "$({x}^{2}) + $({2}^{2})");
    }

    #[tokio::test]
    async fn power_n_power_n_fraction() {
        let mut mala = MaLa::new();
        mala.format("x^2 + 2^2\n((x+x)^2)/2").await;
        assert_eq!(
            mala.content,
            "$({x}^{2}) + $({2}^{2})\n$({($({(x+x)}^{2}))}/{2})"
        );
    }

    #[tokio::test]
    async fn power_n_frac() {
        let mut mala = MaLa::new();
        mala.format("x^2/2").await;
        assert_eq!(mala.content, "$({$({x}^{2})}/{2})");
    }
}
