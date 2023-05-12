use regex::{Regex, Captures};

pub fn preprocess_wikilinks(source: &str, mut link_format: impl FnMut(String, String) -> String) -> String {
    let wiki_regex_double = Regex::new(r"\[\[(.*?)\|(.*?)\]\]").unwrap();
    let wiki_regex_simple = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    let first_replacement = wiki_regex_double
        .replace_all(source, |cap: &Captures| 
                     link_format(cap[1].to_string(), cap[2].to_string())
        ).to_string();

    let second_replacement = wiki_regex_simple
        .replace_all(&first_replacement, "[$1]($1)")
        .to_string();

    second_replacement
}
