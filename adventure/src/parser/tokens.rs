/// A validated single word string of text to be processed by the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
}

/// Prepares a string for parsing by removing invalid formatting.
pub fn tokenize(input: &str) -> Vec<Token> {
    input
        .to_lowercase()
        .split_whitespace()
        .flat_map(|w| {
            // Split by comma and keep comma as separate token.
            // This is important because lists of nouns may be separated by commas.
            let mut parts = Vec::new();
            let mut current = String::new();

            for c in w.chars() {
                if c == ',' {
                    if !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                    parts.push(",".to_string());
                } else if c.is_alphanumeric() {
                    current.push(c);
                }
            }

            if !current.is_empty() {
                parts.push(current);
            }

            parts
        })
        .filter(|w| !w.is_empty())
        .map(|w| Token { text: w })
        .collect()
}
