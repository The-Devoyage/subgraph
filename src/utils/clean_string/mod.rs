#[derive(Debug, Clone)]
pub struct CleanOptions {
    pub newline: Option<bool>,
    pub quotes: Option<bool>,
}

pub fn clean_string(v: &String, options: Option<CleanOptions>) -> String {
    let mut v = v.clone();
    let opts = if options.is_none() {
        CleanOptions {
            newline: Some(true),
            quotes: Some(true),
        }
    } else {
        options.unwrap()
    };
    if opts.newline.unwrap_or(false) {
        v = v.replace("\n", "")
    }
    if opts.quotes.unwrap_or(false) {
        v = v.replace("\"", "")
    }
    v
}
