pub fn normalize_date(date_str: &str) -> String {
    // Expected format: DD.MM.YYYY
    let parts: Vec<&str> = date_str.split('.').collect();
    if parts.len() == 3 {
        let day = parts[0];
        let month = parts[1];
        let year = parts[2];
        if day.len() <= 2 && month.len() <= 2 && year.len() == 4 {
            return format!("{}-{:0>2}-{:0>2}", year, month, day);
        }
    }
    date_str.to_string()
}

pub fn none_if_empty(s: Option<String>) -> Option<String> {
    s.filter(|v| !v.trim().is_empty())
}
