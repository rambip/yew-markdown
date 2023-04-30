mod katex;
use markdown::mdast;
pub use markdown::unist::Point;
use yew::prelude::*;
use stylist::Style;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use std::collections::HashMap;
use web_sys::MouseEvent;
use log::info;
use stylist::yew::use_style;

pub use syntect;
pub use markdown::Constructs;

macro_rules! raw_html {
    ($arg:expr) => {
        Html::from_html_unchecked(AttrValue::from($arg))
    }
}



fn render_header(depth: u8, content: Html) -> Html {
    match depth {
        1 => html!{<h1>{content}</h1>},
        2 => html!{<h2>{content}</h2>},
        3 => html!{<h3>{content}</h3>},
        4 => html!{<h4>{content}</h4>},
        5 => html!{<h5>{content}</h5>},
        6 => html!{<h6>{content}</h6>},
        7 => html!{<h7>{content}</h7>},
        8 => html!{<h8>{content}</h8>},
        _ => html!{<div>{content}</div>},
    }
}

fn render_table(children: &Vec<mdast::Node>, align: &Vec<mdast::AlignKind>) -> Html {
    todo!()
}

fn highlight_code(code: &mdast::Code) -> Option<Html> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.light"];
    let lang = code.lang.clone()?;
    info!("{:?}", ss);
    info!("{}", lang);
    let highlighted_code = syntect::html::highlighted_html_for_string(
        &code.value, 
        &ss, 
        ss.find_syntax_by_token(&lang)?,
        theme).ok()?;
    Some(html!{
        <code>{raw_html!(highlighted_code)}</code>
    })
}

fn render_node(node: &mdast::Node, onclick: &Option<Callback<MarkdownMouseEvent>>) -> Html {
    macro_rules! render_children {
        ($arg:expr) => ($arg.children.iter().map(|x| render_node(x, onclick)))
    }
    match node {
        mdast::Node::Math(m) => raw_html!(katex::render_to_string(&m.value)),
        mdast::Node::InlineMath(m) => raw_html!(katex::render_to_string(&m.value)),
        mdast::Node::Root(n) => render_children!(n).collect::<Html>(),
        mdast::Node::BlockQuote(n) => html!{
            <blockquote>{ for render_children!(n)}</blockquote>
        },
        mdast::Node::FootnoteDefinition(_) => todo!(),
        mdast::Node::List(n) if n.ordered => html!{
            <ol>{for render_children!(n)} </ol>
        },
        mdast::Node::List(n) => html!{
            <ul>{for render_children!(n)} </ul>
        },
        mdast::Node::Break(_) => html!{<br/>},
        mdast::Node::Delete(n) => html!{
            <strike>{for render_children!(n)}</strike>
        },
        mdast::Node::Emphasis(n) => html!{
            <i>{for render_children!(n)}</i>
        },
        mdast::Node::Html(n) => raw_html!(n.value.clone()),
        mdast::Node::Image(n) => html!{
            <img src={n.url.clone()} alt={n.alt.clone()}/>
        },
        // TODO: que sont les enfants ici ?
        mdast::Node::Link(n) => html!{
            <a href={n.url.clone()}>{n.title.clone()}</a>
        },
        mdast::Node::LinkReference(_) => todo!(),
        mdast::Node::Strong(n) => html!{
            <b>{for render_children!(n)}</b>
        },
        mdast::Node::Text(n) => {
            let position = n.position.as_ref().expect("no position on syntax tree").clone();

            if let Some(onclick) = onclick {
                let onclick = onclick.clone();
                let callback = Callback::from(move |x| {
                    let click_event = MarkdownMouseEvent {
                        mouse_event: x,
                        start_position: position.start.clone(),
                        end_position: position.end.clone(),
                    };
                    onclick.emit(click_event)
                });
                html!{
                    <div onclick={callback}>{n.value.clone()}</div>
                }
            }
            else{
                html!{ {n.value.clone()} }
            }
        },
        mdast::Node::Heading(n) => render_header(n.depth, render_children!(n).collect::<Html>()),
        mdast::Node::Table(n) => render_table(&n.children, &n.align),
        mdast::Node::ThematicBreak(_) => html!{<hr/>},
        mdast::Node::TableRow(n) => html!{
            <tr>{for render_children!(n)}</tr>
        },
        mdast::Node::TableCell(n) => html!{
            <td>{for render_children!(n)}</td>
        },
        mdast::Node::ListItem(n) => render_children!(n).collect::<Html>(),
        mdast::Node::Definition(_) => todo!(),
        mdast::Node::Paragraph(n) => html!{<p>{for render_children!(n)}</p>},
        mdast::Node::InlineCode(n) => html!{<code>{n.value.clone()}</code>},
        mdast::Node::Code(c) => match highlight_code(c) {
            Some(html) => html,
            None => html!{<code>{c.value.clone()}</code>},
        }

        // TODO
        mdast::Node::FootnoteReference(_) => html!{"footnote: not implemented"},
        mdast::Node::ImageReference(_) => html!{"image ref: not implemented"},

        // invisible
        mdast::Node::Toml(_) |
        mdast::Node::Yaml(_) => html!{},

        mdast::Node::MdxJsxTextElement(_) |
        mdast::Node::MdxTextExpression(_) |
        mdast::Node::MdxjsEsm(_) |
        mdast::Node::MdxJsxFlowElement(_) |
        mdast::Node::MdxFlowExpression(_) => html!{"this part contain Mdx syntax, not yet implemented"}
    }
}

pub struct Markdown {
    // style: Style,
    cached_ast: HashMap<String, mdast::Node>,
}

#[derive(Clone)]
pub struct MarkdownMouseEvent {
    mouse_event: MouseEvent,
    start_position: Point,
    end_position: Point,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub source: String,
    pub constructs: Option<Constructs>,
    pub onclick: Option<Callback<MarkdownMouseEvent>>,
    // syntax_highlighting_theme: Option<syntect::highlighting::Theme>,
}


impl Component for Markdown {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let content = ctx.props().source.clone();
        let options = markdown::ParseOptions{
            constructs: ctx.props().constructs.clone().unwrap_or(markdown::Constructs::default()),
            gfm_strikethrough_single_tilde: true,
            math_text_single_dollar: true,
            mdx_expression_parse: None,
            mdx_esm_parse: None,
        };
        info!("katex css length: {}", katex::KATEX_CSS.len());
        let ast = markdown::to_mdast(&content, &options)
            .expect("unable to parse markdown");
        // let empty_style = Style::new("").unwrap();
        // FIXME
        // let style = Style::new(katex::KATEX_CSS).expect("unable to read css");
        // let style = use_style!(include_str!("../katex/katex.css"));

        let mut cached_ast = HashMap::new();
        cached_ast.insert(content, ast);
        Self {
            cached_ast,
            // style,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let katex_css_class_name = self.style.get_class_name().to_string();
        let ast = self.cached_ast.get(&ctx.props().source)
            .expect("this markdown text was never compiled");
        info!("{:?}", ast);
        html!{
            <div>{render_node(ast, &ctx.props().onclick)}</div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Props) -> bool {
        if ctx.props() == old_props {
            false
        }
        else {
            let content = ctx.props().source.clone();
            let options = markdown::ParseOptions{
                constructs: ctx.props().constructs.clone().unwrap_or(markdown::Constructs::default()),
                gfm_strikethrough_single_tilde: true,
                math_text_single_dollar: true,
                mdx_expression_parse: None,
                mdx_esm_parse: None,
            };
        let ast = markdown::to_mdast(&content, &options)
            .expect("unable to parse markdown");
            self.cached_ast.insert(content, ast);
            true
        }
    }
}
