pub fn clean_title(title: &str) -> String {

    let no_parens = if let Some(idx) = title.find(['(', '[']) {
        title[..idx].trim()
    } else {
        title.trim()
    };


    let lower = no_parens.to_lowercase();
    let cutoff = lower.find(" feat.")
        .or_else(|| lower.find(" ft."))
        .or_else(|| lower.find(" featuring"));

    match cutoff {
        Some(idx) => no_parens[..idx].trim().to_string(),
        None => no_parens.to_string(),
    }
    }
