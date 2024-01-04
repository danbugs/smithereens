pub fn calculate_spr_or_uf(a: i32, b: i32) -> i32 {
    let s = if a == 1 {
        0.
    } else {
        (a as f32 - 1.).log2().floor() + (2. / 3. * a as f32).log2().ceil()
    };

    let p = if b == 1 {
        0.
    } else {
        (b as f32 - 1.).log2().floor() + (2. / 3. * b as f32).log2().ceil()
    };

    (s - p) as i32
}

pub fn parse_text_vector(text: &str) -> Vec<String> {
    // remove the first and last characters
    let text = &text[1..text.len() - 1];

    if text.is_empty() {
        return Vec::new();
    }

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

pub fn create_page_numbers(curr_page: usize, tot_pages: usize) -> Vec<usize> {
    let mut numbers = Vec::new();

    // Check the viewport width using JavaScript
    let window_size = if is_mobile_viewport() { 3 } else { 5 };
    let half_window = window_size / 2;

    // Calculate the start and end of the pagination window
    let start = if curr_page > half_window {
        curr_page.saturating_sub(half_window)
    } else {
        1
    };
    let end = usize::min(start + window_size - 1, tot_pages);

    // Push your pagination numbers, handling the ellipsis as needed
    if start > 1 {
        numbers.push(1);
        if start > 2 {
            numbers.push(0); // 0 for ellipsis
        }
    }

    numbers.extend(start..=end);

    if end < tot_pages {
        if end < tot_pages - 1 {
            numbers.push(0); // 0 for ellipsis
        }
        numbers.push(tot_pages);
    }

    numbers
}

// Helper function to determine if the viewport is mobile-sized
fn is_mobile_viewport() -> bool {
    // Using web_sys to check the inner width of the window
    if let Some(window) = web_sys::window() {
        if let Ok(width) = window.inner_width() {
            if let Some(width) = width.as_f64() {
                return width <= 768.0; // Use a mobile viewport width threshold, e.g., 768px
            }
        }
    }
    false
}
