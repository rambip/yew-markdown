use markdown::{Constructs, ParseOptions, mdast};
use regex::Regex;

pub fn new_parse_options(constructs: Option<markdown::Constructs>) -> ParseOptions {
    let default_constructs = Constructs {
        // enabled by default
        html_flow: true,
        list_item: true,
        math_flow: true,
        math_text: true,
        attention: true,
        thematic_break: true,
        frontmatter: true,
        block_quote: true,
        character_escape: true,
        code_fenced: true,
        code_text: true,
        character_reference: true,
        gfm_strikethrough: true,
        autolink: true,
        gfm_autolink_literal: true,
        heading_atx: true,
        heading_setext: true,
        label_start_image: true,
        label_start_link: true,
        label_end: true,
        gfm_table: true,
        // see `preprocess_hardbreaks`
        hard_break_trailing: true,

        // TODO
        definition: false,
        gfm_task_list_item: false,
        gfm_footnote_definition: false,
        gfm_label_start_footnote: false,

        // maybe one day
        mdx_esm: false,
        mdx_expression_flow: false,
        mdx_expression_text: false,
        mdx_jsx_flow: false,
        mdx_jsx_text: false,

        // not supported by default
        code_indented: false,
        hard_break_escape: false,
        html_text: false,

    };
    ParseOptions{
        constructs: constructs.clone().unwrap_or(default_constructs),
        gfm_strikethrough_single_tilde: true,
        math_text_single_dollar: true,
        mdx_expression_parse: None,
        mdx_esm_parse: None,
    }
}

fn preprocess_wikilinks(source: &str) -> String {
    let wiki_regex_double = Regex::new(r"\[\[(.*?)\|(.*?)\]\]").unwrap();
    let wiki_regex_simple = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    let first_replacement = wiki_regex_double
        .replace_all(source, "[$2]($1 \"wiki\")")
        .to_string();

    let second_replacement = wiki_regex_simple
        .replace_all(&first_replacement, "[$1]($1)")
        .to_string();

    second_replacement
}

/// hack to allow $$x$$ to be parsed as `Node::Math` and not `Node::MathInline`
fn preprocess_math(source: &str) -> String {
    let math_regex = Regex::new(r"\$\$").unwrap();
    math_regex.replace_all(source, "\n$1\n").to_string()
}

/// hack to add hardbreaks on every line
fn preprocess_hardbreaks(source: &str) -> String {
    source.replace("\n", "\n  ")
}

pub fn parse(source: &str, parse_options: &markdown::ParseOptions, wikilinks: bool) -> mdast::Node {
    let mut source = source.to_string();
    if wikilinks {
        source = preprocess_wikilinks(&source);
    }
    source = preprocess_math(&source);
    source = preprocess_hardbreaks(&source);
    markdown::to_mdast(&source, parse_options).expect("unable to parse markdown")
}

