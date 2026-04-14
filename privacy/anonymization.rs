pub struct Anonymizer;

impl Anonymizer {
    pub fn strip_pii(data: &str) -> String {
        let mut cleaned = data.to_string();

        // VERY basic placeholder rules
        cleaned = cleaned.replace("@", "[at]");
        cleaned = cleaned.replace(".com", "[dot]com");

        cleaned
    }

    pub fn mask(data: &str) -> String {
        if data.len() <= 4 {
            return "*".repeat(data.len());
        }

        format!(
            "{}****{}",
            &data[..2],
            &data[data.len() - 2..]
        )
    }
}
