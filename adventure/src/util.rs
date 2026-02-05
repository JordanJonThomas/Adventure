/// Produces a string with the first letter capitalized.
pub(crate) fn capitalize(canonical: &str) -> String {
        // Capitalize first letter
        let mut chars = canonical.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
}
