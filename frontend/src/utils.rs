pub fn calculate_spr(seed: i32, placement: i32) -> i32 {
    let s = if seed == 1 {
        0.
    } else {
        (seed as f32 - 1.).log2().floor() + (2./3. * seed as f32).log2().ceil()
    };
    
    let p = if placement == 1 {
        0.
    } else {
        (placement as f32 - 1.).log2().floor() + (2./3. * placement as f32).log2().ceil()
    };

    (s - p) as i32
}

pub fn parse_text_vector(text: &str) -> Vec<String> {
    // remove the first and last characters
    let text = &text[1..text.len() - 1];
    
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for c in text.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            result.push(current);
            current = String::new();
        } else {
            current.push(c);
        }
    }
    result.push(current);
    result
}