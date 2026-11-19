fn url_encode_latex(latex: &str) -> String {
    let mut encoded = String::with_capacity(latex.len() * 2);
    for b in latex.trim().bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' | b'\n' | b'\r' | b'\t' => {
                encoded.push_str("%20");
            }
            _ => {
                encoded.push_str(&format!("%{b:02X}"));
            }
        }
    }
    encoded
}

pub fn format_latex_blocks(text: &str) -> String {
    if !text.contains("$$") {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut parts = text.split("$$");

    if let Some(first) = parts.next() {
        result.push_str(first);
    }

    while let Some(formula) = parts.next() {
        if let Some(after_formula) = parts.next() {
            let trimmed = formula.trim();
            if !trimmed.is_empty() {
                let encoded = url_encode_latex(trimmed);
                let url = format!(
                    "https://latex.codecogs.com/png.image?%5Cdpi%7B200%7D%5Cfg%7Bwhite%7D%5Cbg%7B313338%7D%20{encoded}"
                );
                result.push_str(&format!("[📐 Fórmula]({url})"));
            }
            result.push_str(after_formula);
        } else {
            result.push_str("$$");
            result.push_str(formula);
        }
    }

    result
}
