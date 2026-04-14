use std::collections::HashMap;

pub struct SelectiveDisclosure;

impl SelectiveDisclosure {
    pub fn filter_fields(
        data: HashMap<String, String>,
        allowed_fields: Vec<String>,
    ) -> HashMap<String, String> {
        data.into_iter()
            .filter(|(k, _)| allowed_fields.contains(k))
            .collect()
    }

    pub fn redact(data: &str, keywords: &[&str]) -> String {
        let mut output = data.to_string();

        for key in keywords {
            output = output.replace(key, "[REDACTED]");
        }

        output
    }
}
