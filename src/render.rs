use yew::{Html, html, Callback, virtual_dom::AttrValue};
use katex;
use markdown::mdast::*;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};

use super::mouse_event;

pub struct LinkProps {
    pub name: AttrValue,
    pub alias: AttrValue,
    pub title: Option<AttrValue>,
}

/// all the context needed to render markdown:
/// - a `syntax_set` and a `theme` for syntax highlighting
/// - a callback `onclick` to add interactivity
pub struct RenderContext {
    syntax_set: SyntaxSet,
    theme: Theme,
    onclick: Callback<mouse_event::MarkdownMouseEvent>,
    render_links: Callback<LinkProps, Html>,
}

impl RenderContext {
    pub fn new(theme_name: Option<String>, 
               onclick: Option<Callback<mouse_event::MarkdownMouseEvent>>,
               render_links: Option<Callback<LinkProps, Html>>) -> Self {

        let theme_set = ThemeSet::load_defaults();
        let theme_name = theme_name
            .unwrap_or("base16-ocean.light".to_string());
        let theme = theme_set.themes.get(&theme_name)
            .expect("unknown theme")
            .clone();

        let syntax_set = SyntaxSet::load_defaults_newlines();

        RenderContext {
            syntax_set,
            theme,
            onclick : onclick.unwrap_or(Callback::from(|_| ())),
            render_links: render_links.unwrap_or(Callback::from(render_links_default)),
        }
    }
}


/// convert the input string or &str directly to an html node
macro_rules! raw_html {
    ($arg:expr) => {
        Html::from_html_unchecked(AttrValue::from($arg))
    }
}

/// convert the input string or &str directly to an html node
macro_rules! html_error {
    ($arg:expr) => {
        html!{
            <span class="markdown-error">{$arg}</span>
        }
    }
}


/// `render_maths(content)` returns a html node
/// with the latex content `content` compiled inside
fn render_maths(content: &str) -> Option<Html>{
    katex::render(content)
        .ok()
        .map(|x| raw_html!(x))
}

/// `highlight_code(content, ss, ts)` render the content `content`
/// with syntax highlighting
fn highlight_code(content: &str, lang: Option<&str>, context: &RenderContext) -> Option<Html> {
    Some( raw_html!(
        syntect::html::highlighted_html_for_string(
            content,
            &context.syntax_set, 
            context.syntax_set.find_syntax_by_token(lang?)?,
            &context.theme
            ).ok()?
    ))
}

/// `render_header(d, s)` returns the html corresponding to
/// the string `s` inside a html header with depth `d`
fn render_header(depth: u8, content: Html) -> Html {
    match depth {
        1 => html!{<h1>{content}</h1>},
        2 => html!{<h2>{content}</h2>},
        3 => html!{<h3>{content}</h3>},
        4 => html!{<h4>{content}</h4>},
        5 => html!{<h5>{content}</h5>},
        6 => html!{<h6>{content}</h6>},
        _ => panic!("maximum heading level exceeded")
    }
}

/// `align_string(align)` gives the css string
/// that is used to align text according to `align`
fn align_string(align: &AlignKind) -> &'static str {
    match align {
        AlignKind::Left => "text-align: left",
        AlignKind::Right => "text-align: right",
        AlignKind::Center => "text-align: center",
        AlignKind::None => "",
    }
}

/// `render_table_row(row, align, context)` renders the markdown element `row`
/// to html using `align` to align each cell.
/// `context` is used to render the child components
fn render_table_row<'a> (row: &'a TableRow, align: &Vec<AlignKind>, context: &RenderContext) -> Html {
    use core::iter::zip;
    let unwrap_cell = |node: &'a Node| match node {
        Node::TableCell(t) => t, 
        _ => panic!("table row contains something that is not a cell"),
    };

    let render_cell = |(cell, align): (&'a Node, &AlignKind)| html!{
        <td style={align_string(align)}>
            {for unwrap_cell(cell).children.iter().map(|x| render_node(x, context))}
        </td>
    };

    html!{
        <tr>
            {for zip(&row.children, align).map(render_cell)}
        </tr>
    }
}

fn render_links_default(link: LinkProps) -> Html {
    html!{
        <a href={link.name}>{link.alias}</a>
    }
}


/// `render_node(node, context)` returns an html view
/// of the markdown abstract syntax tree `node`.
/// It uses all the context present in `context`
pub fn render_node<'a>(node: &'a Node, context: &RenderContext) -> Html {
    macro_rules! render_children {
        ($arg:expr) => ($arg.children.iter().map(|x| render_node(x, context)))
    }
    match node {
        Node::Html(n) => raw_html!(n.value.clone()),

        Node::Text(n) => html!{
            <span onclick={mouse_event::make_callback(&context.onclick, &n.position)}>
            {n.value.clone()}
            </span>
        },

        Node::Root(n) => render_children!(n).collect::<Html>(),

        Node::BlockQuote(n) => html!{
            <blockquote> { for render_children!(n)} </blockquote>
        },

        Node::FootnoteDefinition(_) => todo!(),


        Node::Break(_) => html!{<br/>},
        Node::Delete(n) => html!{
            <strike>{for render_children!(n)}</strike>
        },
        Node::Emphasis(n) => html!{
            <i>{for render_children!(n)}</i>
        },
        Node::Strong(n) => html!{
            <b>{for render_children!(n)}</b>
        },

        Node::Heading(n) => render_header(
            n.depth, 
            render_children!(n).collect::<Html>()
        ),
        Node::ThematicBreak(_) => html!{<hr/>},
        Node::Paragraph(n) => html!{<p>{for render_children!(n)}</p>},

        Node::List(n) if n.ordered => html!{
            <ol start={n.start.unwrap_or(0).to_string()}>
            {for render_children!(n)} 
            </ol>
        },

        Node::List(n) => html!{
            <ul> {for render_children!(n)} </ul>
        },
        Node::ListItem(n) => html!{
            <li>{for render_children!(n)}</li>
        },

        Node::TableRow(n) => html!{
            <tr>{for render_children!(n)}</tr>
        },
        Node::TableCell(n) => html!{
            <td>{for render_children!(n)}</td>
        },

        Node::Image(n) => html!{
            <img src={n.url.clone()} alt={n.alt.clone()}/>
        },
        // TODO: what to do about `n.title` ?
        Node::Link(n) => match &n.children[..] {
            [Node::Text(t)] => context.render_links.emit(LinkProps{
                name : n.url.clone().into(),
                alias : t.value.clone().into(),
                title: n.title.clone().map(|x| x.into()),
            }),
            _ => html_error!("markdown content in a link is not allowed"),

        },

        Node::InlineCode(n) => html!{
            <code onclick={mouse_event::make_callback(&context.onclick, &n.position)}>
                {n.value.clone()}
            </code>
        },
        Node::Code(c) => {
            let code_content = highlight_code(&c.value, c.lang.as_deref(), context) 
                .unwrap_or_else(|| html!{c.value.clone()});

            html!{
                <code onclick={mouse_event::make_callback(&context.onclick, &c.position)}>
                {code_content}
                </code>
            }
        },

        Node::Math(m) => html!(
            <div class={"math-flow"} onclick={mouse_event::make_callback(&context.onclick, &m.position)}>
            {render_maths(&m.value).unwrap_or(html_error!{"invalid math"})}
            </div>
            ),
        Node::InlineMath(m) => html!(
            <span class={"math-inline"} onclick={mouse_event::make_callback(&context.onclick, &m.position)}>
            {render_maths(&m.value).unwrap_or(html_error!{"invalid math"})}
            </span>
            ),

        Node::Table(t) => {
            let unwrap_row = |node: &'a Node| match node {
                Node::TableRow(x) => x,
                _ => panic!("the table contains something that is not a row"),
            };
            html!{
                <table>
                    {for t.children.iter()
                        .map(|c| render_table_row(&unwrap_row(c), &t.align, context))}
                </table>
            }
        },
        Node::FootnoteReference(_) => html_error!{"footnote: not implemented"},
        Node::ImageReference(_) => html_error!{"image ref: not implemented"},
        Node::LinkReference(_) => html_error!{"link ref: not implemented"},
        Node::Definition(_) => html_error!{"definition: not implemented"},

        // invisible
        Node::Toml(_) |
        Node::Yaml(_) => html!{},

        Node::MdxJsxTextElement(_) |
        Node::MdxTextExpression(_) |
        Node::MdxjsEsm(_) |
        Node::MdxJsxFlowElement(_) |
        Node::MdxFlowExpression(_) => html_error!{"this part contain Mdx syntax, not yet implemented"}
    }
}
