// mod katex;
use katex;
use markdown::mdast;
pub use markdown::unist::Point;
use yew::{prelude::*, virtual_dom::AttrValue};
use stylist::{StyleSource, css};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};
use std::collections::HashMap;
use web_sys::MouseEvent;
use log::info;

pub use syntect;
pub use markdown::Constructs;

/// convert the input string or &str directly to an html node
macro_rules! raw_html {
    ($arg:expr) => {
        Html::from_html_unchecked(AttrValue::from($arg))
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
fn highlight_code(content: &str, lang: Option<&str>, ss: &SyntaxSet, theme: &Theme) -> Option<Html> {
    Some( raw_html!(
        syntect::html::highlighted_html_for_string(
            content,
            &ss, 
            ss.find_syntax_by_token(lang?)?,
            theme).ok()?
    ))
}

fn render_header(depth: u8, content: Html) -> Html {
    match depth {
        1 => html!{<h1>{content}</h1>},
        2 => html!{<h2>{content}</h2>},
        3 => html!{<h3>{content}</h3>},
        4 => html!{<h4>{content}</h4>},
        5 => html!{<h5>{content}</h5>},
        6 => html!{<h6>{content}</h6>},
        _ => html!{<div>{content}</div>},
    }
}

fn render_table(children: &Vec<mdast::Node>, align: &Vec<mdast::AlignKind>) -> Html {
    todo!()
}

/// `make_markdown_mouse_event_callback(onclick, position)` 
/// composes the callback `onclick` with a converter
/// to get a usable callback for the user
fn make_markdown_mouse_event_callback(onclick: &Option<Callback<MarkdownMouseEvent>>, position: &Option<markdown::unist::Position>) 
    -> Callback<MouseEvent> {
    let position = position.clone().expect("unable to know from which position the markdown tree was build");
    let onclick = onclick.clone();
    match onclick {
        Some(callback) =>{
            Callback::from(move |x| {
                let click_event = MarkdownMouseEvent {
                    mouse_event: x,
                    start_position: position.start.clone(),
                    end_position: position.end.clone(),
                };
                callback.emit(click_event)
            })
        },
        None => Callback::from(|_| ())
    }
}

/// `render_node(node, onclick, syntax_set, theme_set` returns an html view
/// of the markdown abstract syntax tree `node`.
/// It uses a click callback `onclick`, a syntax set and a theme for theming
fn render_node(node: &mdast::Node, onclick: &Option<Callback<MarkdownMouseEvent>>, 
               syntax_set: &SyntaxSet, theme: &Theme) -> Html {
    macro_rules! render_children {
        ($arg:expr) => ($arg.children.iter().map(|x| render_node(x, onclick, syntax_set, theme)))
    }
    match node {
        mdast::Node::Html(n) => raw_html!(n.value.clone()),

        mdast::Node::Text(n) => html!{
            <span onclick={make_markdown_mouse_event_callback(&onclick, &n.position)}>
            {n.value.clone()}
            </span>
        },

        mdast::Node::Root(n) => render_children!(n).collect::<Html>(),

        mdast::Node::BlockQuote(n) => html!{
            <blockquote> { for render_children!(n)} </blockquote>
        },

        mdast::Node::FootnoteDefinition(_) => todo!(),


        mdast::Node::Break(_) => html!{<br/>},
        mdast::Node::Delete(n) => html!{
            <strike>{for render_children!(n)}</strike>
        },
        mdast::Node::Emphasis(n) => html!{
            <i>{for render_children!(n)}</i>
        },
        mdast::Node::Strong(n) => html!{
            <b>{for render_children!(n)}</b>
        },

        mdast::Node::Heading(n) => render_header(
            n.depth, 
            render_children!(n).collect::<Html>()
        ),
        mdast::Node::ThematicBreak(_) => html!{<hr/>},
        mdast::Node::Paragraph(n) => html!{<p>{for render_children!(n)}</p>},

        mdast::Node::List(n) if n.ordered => html!{
            <ol start={n.start.unwrap_or(0).to_string()}>
            {for render_children!(n)} 
            </ol>
        },

        mdast::Node::List(n) => html!{
            <ul> {for render_children!(n)} </ul>
        },
        mdast::Node::ListItem(n) => html!{
            <li>{for render_children!(n)}</li>
        },

        mdast::Node::TableRow(n) => html!{
            <tr>{for render_children!(n)}</tr>
        },
        mdast::Node::TableCell(n) => html!{
            <td>{for render_children!(n)}</td>
        },

        mdast::Node::Image(n) => html!{
            <img src={n.url.clone()} alt={n.alt.clone()}/>
        },
        // TODO: what to do about `n.title` ?
        mdast::Node::Link(n) => html!{
            <a href={n.url.clone()}>
            {for render_children!(n)}
            </a>
        },

        mdast::Node::InlineCode(n) => html!{
            <code onclick={make_markdown_mouse_event_callback(&onclick, &n.position)}>
                {n.value.clone()}
            </code>
        },
        mdast::Node::Code(c) => {
            let code_content = highlight_code(&c.value, c.lang.as_deref(), 
                                              syntax_set, theme)
                .unwrap_or_else(|| html!{c.value.clone()});

            html!{
                <code onclick={make_markdown_mouse_event_callback(&onclick, &c.position)}>
                {code_content}
                </code>
            }
        },

        mdast::Node::Math(m) => html!(
            <div onclick={make_markdown_mouse_event_callback(&onclick, &m.position)}>
            {render_maths(&m.value).unwrap_or(html!{"invalid math"})}
            </div>
            ),
        mdast::Node::InlineMath(m) => html!(
            <div onclick={make_markdown_mouse_event_callback(&onclick, &m.position)}>
            {render_maths(&m.value).unwrap_or(html!{"invalid math"})}
            </div>
            ),

        // TODO
        mdast::Node::Table(n) => render_table(&n.children, &n.align),
        mdast::Node::FootnoteReference(_) => html!{"footnote: not implemented"},
        mdast::Node::ImageReference(_) => html!{"image ref: not implemented"},
        mdast::Node::LinkReference(_) => html!{"link ref: not implemented"},
        mdast::Node::Definition(_) => html!{"definition: not implemented"},

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

/// the markdown component
pub struct Markdown {
    ast: mdast::Node,
    cached_ast: HashMap<AttrValue, mdast::Node>,
    parse_options: markdown::ParseOptions,
    theme: Theme,
    syntax_set: SyntaxSet,
    style: StyleSource,
}

/// `mouse_event` -> the original mouse event triggered when a text element was clicked on
/// `start_position: Point` -> the corresponding starting position in the markdown source
/// `end_position: Point` -> the corresponding ending position in the markdown source
#[derive(Clone, Debug)]
pub struct MarkdownMouseEvent {
    pub mouse_event: MouseEvent,
    pub start_position: Point,
    pub end_position: Point,
}

/// Properties for `Markdown`
/// `src` -> the raw markdown content to render
/// `constructs` -> the markdown 
/// `onclick`: callback executed when a portion of the markdown document  is clicked
/// `style`: the css of the component. By default there is some minimal styling 
///          added to the html
/// `theme_name`: the name of the theme for `syntect`. Refer to their docs
/// `caching`: if set to true, the syntax trees will be cached. 
///     That means that if your render document A, then B, then A, 
///     document A will not have to be parsed a second time
#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    pub src: AttrValue,
    pub constructs: Option<Constructs>,
    pub onclick: Option<Callback<MarkdownMouseEvent>>,
    pub style: Option<StyleSource>,
    pub theme_name: Option<String>,

    #[prop_or(false)]
    pub caching: bool,
}


impl Component for Markdown {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let default_constructs = markdown::Constructs {
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

            // TODO
            gfm_table: false,
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
            hard_break_trailing: false,
            html_text: false,

        };
        let parse_options = markdown::ParseOptions{
            constructs: ctx.props().constructs.clone().unwrap_or(default_constructs),
            gfm_strikethrough_single_tilde: true,
            math_text_single_dollar: true,
            mdx_expression_parse: None,
            mdx_esm_parse: None,
        };

        let style = ctx.props().style.clone().unwrap_or(
            css!(r"
                 blockquote{ margin: 20px; border-left : 10px solid grey} 
                 ul {list-style-type:disc}
                 "
            )
        );

        let theme_set = ThemeSet::load_defaults();
        let theme_name = ctx.props().theme_name.clone()
            .unwrap_or("base16-ocean.light".to_string());
        let theme = theme_set.themes.get(&theme_name)
            .expect("unknown theme")
            .clone();

        Self {
            cached_ast: HashMap::new(),
            ast: markdown::to_mdast(&ctx.props().src, &parse_options).unwrap(),
            parse_options,
            syntax_set : SyntaxSet::load_defaults_newlines(),
            theme,
            style,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class={self.style.clone()}>
                { render_node(&self.ast, &ctx.props().onclick, &self.syntax_set, &self.theme)}
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Props) -> bool {
        let content = ctx.props().src.clone();
        let ast = markdown::to_mdast(&content, &self.parse_options)
            .expect("unable to parse markdown");
        let old_ast = std::mem::replace(&mut self.ast, ast);
        if ctx.props().caching {
            self.cached_ast.insert(old_props.src.clone(), old_ast);
        }
        true
    }
}
