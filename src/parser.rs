use markdown::{Constructs, ParseOptions, mdast, mdast::Node};

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

pub fn parse(source: &str, parse_options: &markdown::ParseOptions, wikilinks: bool) -> Node {
    let mut ast = markdown::to_mdast(&source.to_string(), parse_options).expect("unable to parse markdown");
    postprocess(source, &mut ast, wikilinks);
    ast
}

fn postprocess(source: &str, ast: &mut Node, wikilinks: bool) {
    match ast {
        Node::Text(mdast::Text{ value, position}) if wikilinks => {
            *ast = Node::Paragraph(mdast::Paragraph{
                position: position.clone(),
                children: todo!()
            })
        },
        Node::Paragraph(mdast::Paragraph{position: Some(p), ref mut children }) if wikilinks => {
            children = todo!()
        },
        Node::InlineMath(m) if is_inline_latex(source, &m) => 
            *ast = Node::Math(mdast::Math{
                value: m.value.clone(),
                position: m.position.clone(),
                meta: None,
            }),
        x => {
            for c in x.children_mut().into_iter().flatten() {
                postprocess(source, c, wikilinks)
            }
        }
    }
}


fn is_inline_latex(source: &str, m: &mdast::InlineMath) -> bool {
    match &m.position {
        Some(p) => {
            source.get(p.start.offset..p.start.offset+2) == Some("$$") 
            && source.get(p.end.offset-2..p.end.offset) == Some("$$")
        }
        None => false
    }
}

#[cfg(test)]
mod tests {
    use wasm_test::*;

    #[wasm_test]
    fn test_offset(){
        let s = "$$x$$";
        assert!(s.get(0..2)==Some("$$"));
        assert!(s.get(s.len()-2..s.len())==Some("$$"));
    }
}
