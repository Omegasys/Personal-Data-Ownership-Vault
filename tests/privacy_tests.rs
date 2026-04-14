use data_vault::privacy::anonymization::Anonymizer;
use data_vault::privacy::selective_disclosure::SelectiveDisclosure;
use std::collections::HashMap;

#[test]
fn test_anonymization_masking() {
    let input = "john.doe@example.com";
    let masked = Anonymizer::mask(input);

    assert!(masked.contains("**"));
}

#[test]
fn test_strip_pii() {
    let input = "contact me at john@example.com";
    let output = Anonymizer::strip_pii(input);

    assert!(!output.contains("@"));
}

#[test]
fn test_selective_disclosure() {
    let mut data = HashMap::new();
    data.insert("name".into(), "alice".into());
    data.insert("secret".into(), "hidden".into());

    let filtered =
        SelectiveDisclosure::filter_fields(data, vec!["name".into()]);

    assert!(filtered.contains_key("name"));
    assert!(!filtered.contains_key("secret"));
}
