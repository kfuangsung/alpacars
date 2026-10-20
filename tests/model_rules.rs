//! Response models are `#[non_exhaustive]`, so a field Alpaca adds later can ship in a patch
//! release. A model that callers also send as a request body must stay constructible; those are
//! listed here and documented on the type.

const CONSTRUCTIBLE: &[&str] = &["AccountConfiguration"];

#[test]
fn test_response_models_are_non_exhaustive() {
    for file in [
        "src/trading/models.rs",
        "src/broker/models.rs",
        "src/data/models.rs",
    ] {
        let src = std::fs::read_to_string(file).unwrap();
        let lines: Vec<&str> = src.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let Some(rest) = line.strip_prefix("pub struct ") else {
                continue;
            };
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            let attrs: Vec<&str> = lines[..i]
                .iter()
                .rev()
                .take_while(|l| l.starts_with("#[") || l.starts_with("///"))
                .copied()
                .collect();
            let marked = attrs.contains(&"#[non_exhaustive]");
            if CONSTRUCTIBLE.contains(&name.as_str()) {
                assert!(
                    !marked,
                    "{file}: {name} is a request body and must stay constructible"
                );
            } else {
                assert!(
                    marked,
                    "{file}: response model {name} is not #[non_exhaustive]"
                );
            }
        }
    }
}
