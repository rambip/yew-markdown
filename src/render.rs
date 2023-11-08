use yew::{Html, html, Callback};
use core::ops::Range;

use katex;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};

use web_sys::MouseEvent;

use pulldown_cmark_wikilink::{Event, Tag, TagEnd, CodeBlockKind, Alignment, MathMode, HeadingLevel};

use crate::utils::as_closing_tag;
use super::LinkProps;
use super::MarkdownMouseEvent;


/// all the context needed to render markdown:
pub struct RenderContext {
    /// syntax used for syntax highlighting
    syntax_set: SyntaxSet,

    /// theme used for syntax highlighting
    theme: Theme,

    /// callback to add interactivity to the rendered markdown
    onclick: Callback<MarkdownMouseEvent>,

    /// callback used to render links
    render_links: Option<Callback<LinkProps, Html>>,
}

/// `make_markdown_mouse_event_callback(onclick, position)` 
/// composes the callback `onclick` with a converter
/// to get a usable callback for the user
pub fn make_callback(context: &RenderContext, position: Range<usize>) 
    -> Callback<MouseEvent> {
        let onclick = context.onclick.clone();
        Callback::from(move |x| {
            let click_event = MarkdownMouseEvent {
                mouse_event: x,
                position: position.clone()
            };
            onclick.emit(click_event)
    })
}


impl RenderContext
{
    pub fn new(theme_name: Option<String>, 
               onclick: Option<Callback<MarkdownMouseEvent>>,
               render_links: Option<Callback<LinkProps, Html>>,
               )
-> Self 
{
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
            onclick: onclick.unwrap_or(Callback::from(|_| ())),
            render_links,
        }
    }
}


pub struct HtmlError(String);

impl HtmlError {
    fn err<T>(message: &str) -> Result<T, Self>{
        Err(HtmlError(message.to_string()))
    }
}

impl ToString for HtmlError {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}


use Event::*;



pub struct Renderer<'a, 'c, I>
where I: Iterator<Item=(Event<'a>, Range<usize>)>
{
    context: &'a RenderContext,
    stream: &'c mut I,
    // TODO: Vec<Alignment> to &[Alignment] to avoid cloning.
    // But it requires to provide the right lifetime
    column_alignment: Option<Vec<Alignment>>,
    cell_index: usize,
    end_tag: Option<TagEnd>,
    current_component: Option<String>
}


impl<'a, 'c, I> Iterator for Renderer<'a, 'c, I> 
where I: Iterator<Item=(Event<'a>, Range<usize>)>
{
    type Item = Html;

    fn next(&mut self) -> Option<Self::Item> {
        let (item, range) = self.stream.next()? ;
        let range = range.clone();

        let rendered = match item {
            Start(t) => self.render_tag(t, range),
            End(end) => {
                // check if the closing tag is the tag that was open
                // when this renderer was created
                match self.end_tag {
                    Some(t) if t == end => return None,
                    Some(t) => panic!("{t:?} is a wrong closing tag"),
                    None => panic!("didn't expect a closing tag")
                }
            },
            Text(s) => Ok(render_text(self.context, &s, range)),
            Code(s) => Ok(render_code(self.context, &s, range)),
            InlineHtml(s) => Ok(self.html(&s, range)), 
            Html(s) => Ok(self.html(&s, range)),
            FootnoteReference(_) => HtmlError::err("do not support footnote refs yet"),
            SoftBreak => Ok(self.next()?),
            HardBreak => Ok(html!{<br/>}),
            Rule => Ok(render_rule(self.context, range)),
            TaskListMarker(m) => Ok(render_tasklist_marker(self.context, m, range)),
            Math(disp, content) => render_maths(self.context, &content, &disp, range),
        };

        Some(
            rendered.unwrap_or_else(|e| html!{
                <span class="error" style="border: 1px solid red">
                    {e.to_string()}
                    <br/>
                </span>
                }
            )
        )
    }
}


impl<'a, 'c, I> Renderer<'a, 'c, I> 
where I: Iterator<Item=(Event<'a>, Range<usize>)>
{
    pub fn new(context: &'a RenderContext, events: &'c mut I)-> Self 
    {
        Self {
            context,
            stream: events,
            column_alignment: None,
            cell_index: 0,
            end_tag: None,
            current_component: None,
        }
    }

    fn html(&mut self, s: &str, _range: Range<usize>) 
        -> Html {
            html!{
                <span inner_html={s.to_string()}></span>
            }
    }

    fn children(&mut self, tag: Tag<'a>) -> Html {
        let sub_renderer = Renderer {
            context: self.context,
            stream: self.stream,
            column_alignment: self.column_alignment.clone(),
            cell_index: 0,
            end_tag: Some(as_closing_tag(&tag)),
            current_component: self.current_component.clone()
        };
        sub_renderer.collect::<Html>()
    }

    fn children_text(&mut self, tag: Tag<'a>) -> Option<String> {
        let text = match self.stream.next() {
            Some((Event::Text(s), _)) => Some(s.to_string()),
            None => None,
            _ => panic!("expected string event, got something else")
        };

        let end_tag = &self.stream.next().expect("this event should be the closing tag").0;
        assert!(end_tag == &Event::End(as_closing_tag(&tag)));

        text
    }

    fn children_html(&mut self, tag: Tag<'a>) -> Option<String> {
        let text = match self.stream.next() {
            Some((Event::Html(s), _)) => Some(s.to_string()),
            None => None,
            _ => panic!("expected html event, got something else")
        };

        let end_tag = &self.stream.next().expect("this event should be the closing tag").0;
        assert!(end_tag == &Event::End(as_closing_tag(&tag)));

        text
    }

    fn render_tag(&mut self, tag: Tag<'a>, range: Range<usize>) 
    -> Result<Html, HtmlError> 
    {
        Ok(match tag.clone() {
            Tag::HtmlBlock => {
                let child = self.children_html(Tag::HtmlBlock);
                child.map(|raw_html| html!{<div inner_html={raw_html}></div>})
                    .into_iter()
                    .collect::<Html>()
            }
            Tag::Paragraph => html!{<p>{self.children(tag)}</p>},
            Tag::Heading{level, ..} => render_heading(level, self.children(tag))
            ,
            Tag::BlockQuote => html!{
                <blockquote>
                    {self.children(tag)}
                </blockquote>
            },
            Tag::CodeBlock(k) => 
                render_code_block(self.context, self.children_text(tag), &k,range),
            Tag::List(Some(n0)) => html!{
                <ol start={Some(n0.to_string())}>
                    {self.children(tag)}
                </ol>},
            Tag::List(None) => html!{<ul>{self.children(tag)}</ul>},
            Tag::Item => html!{<li>{self.children(tag)}</li>},
            Tag::Table(align) => {
                self.column_alignment = Some(align);
                html!{<table>{self.children(tag)}</table>}
            }
            Tag::TableHead => {
                html!{
                    <thead>{self.children(tag)}</thead>
                }
            },
            Tag::TableRow => {
                html!{
                    <tr>{self.children(tag)}</tr>
                }
            }
            Tag::TableCell => {
                let align = self.column_alignment.clone().unwrap()[self.cell_index];
                self.cell_index += 1;
                render_cell(self.context, self.children(tag), &align)
            }
            Tag::Emphasis => html!{<i>{self.children(tag)}</i>},
            Tag::Strong => html!{<b>{self.children(tag)}</b>},            
            Tag::Strikethrough => html!{<s>{self.children(tag)}</s>},            
            Tag::Image{link_type, dest_url, title, ..} => {
                let description = LinkProps {
                    url: dest_url.to_string(),
                    title: title.to_string(),
                    content: self.children(tag),
                    link_type,
                    image: true,
                };
                render_link(self.context, description)?
            },
            Tag::Link{link_type, dest_url, title, ..} => {
                let description = LinkProps {
                    url: dest_url.to_string(),
                    title: title.to_string(),
                    content: self.children(tag),
                    link_type,
                    image: false,
                };
                render_link(self.context, description)?
            },
            Tag::FootnoteDefinition(_) => return HtmlError::err("footnote: not implemented"),
            Tag::MetadataBlock{..} => {
                let _c = self.children_text(tag);
                html!()
            }
        })
    }
}


fn render_tasklist_marker(context: &RenderContext, m: bool, position: Range<usize>) 
    -> Html {
    let callback = context.onclick.clone();
    let callback = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        let click_event = MarkdownMouseEvent {
            mouse_event: e,
            position: position.clone()
        };
        callback.emit(click_event)
    };
    html!{
        <input type="checkbox" checked={m} onclick={callback}/>
    }
}

fn render_rule(context: &RenderContext, range: Range<usize>) -> Html{
    let callback = make_callback(context, range);
    html!{<hr onclick={callback}/>}
}


fn render_code(context: &RenderContext, s: &str, range: Range<usize>) -> Html {
    let callback = make_callback(context, range);
    html!{<code onclick={callback}>{s.to_string()}</code>}
}

fn render_text(context: &RenderContext, s: &str, range: Range<usize>) -> Html{
    let callback = make_callback(context, range);
    html!{
        <span onclick={callback}>
            {s.to_string()}
        </span>
    }
}


fn render_code_block(context: &RenderContext, 
                     string_content: Option<String>,
                     k: &CodeBlockKind,
                     range: Range<usize>
    ) -> Html {
    let content = match string_content {
        Some(x) => x,
        None => return html!{
            <code></code>
        }
    };

    let callback = make_callback(context, range);

    match highlight_code(context, &content, &k) {
        None => html!{
        <code onclick={callback}>
            <pre inner_html={content.to_string()}></pre>
        </code>
        },
        Some(x) => html!{
            <div onclick={callback} inner_html={x}>
                </div>
        }
    }
}


/// `highlight_code(content, ss, ts)` render the content `content`
/// with syntax highlighting
fn highlight_code(context: &RenderContext, content: &str, kind: &CodeBlockKind) -> Option<String> {
    let lang = match kind {
        CodeBlockKind::Fenced(x) => x,
        CodeBlockKind::Indented => return None
    };
    Some(
        syntect::html::highlighted_html_for_string(
            content,
            &context.syntax_set, 
            context.syntax_set.find_syntax_by_token(lang)?,
            &context.theme
            ).ok()?
    )
}


/// `render_header(d, s)` returns the html corresponding to
/// the string `s` inside a html header with depth `d`
fn render_heading(level: HeadingLevel, content: Html) -> Html {
    use HeadingLevel::*;
    match level {
        H1 => html!{<h1>{content}</h1>},
        H2 => html!{<h2>{content}</h2>},
        H3 => html!{<h3>{content}</h3>},
        H4 => html!{<h4>{content}</h4>},
        H5 => html!{<h5>{content}</h5>},
        H6 => html!{<h6>{content}</h6>},
    }
}


/// `render_maths(content)` returns a html node
/// with the latex content `content` compiled inside
fn render_maths(context: &RenderContext, content: &str, display_mode: &MathMode, range: Range<usize>) 
    -> Result<Html, HtmlError>{
    let opts = katex::Opts::builder()
        .display_mode(*display_mode == MathMode::Display)
        .build()
        .unwrap();

    let class_name = match display_mode {
        MathMode::Inline => "math-inline",
        MathMode::Display => "math-flow",
    };

    let callback = make_callback(context, range);

    match katex::render_with_opts(content, opts){
        Ok(x) => Ok(html!{
            <span inner_html={x} class={class_name} onclick={callback}></span>
        }),
        Err(_) => HtmlError::err("invalid math")
    }
}

fn render_link(context: &RenderContext, link: LinkProps) 
    -> Result<Html, HtmlError> 
{
    match (&context.render_links, link.image) {
        (Some(f), _) => Ok(f.emit(link)),
        (None, false) => Ok(html!{
                <a href={link.url}>
                    {link.content}
                </a>
            }
        ),
        (None, true) => Ok(html!{
                <img src={link.url} alt={link.title}/>
            }
        )
    }
}

/// `align_string(align)` gives the css string
/// that is used to align text according to `align`
fn align_string(align: &Alignment) -> &'static str {
    match align {
        Alignment::Left => "text-align: left",
        Alignment::Right => "text-align: right",
        Alignment::Center => "text-align: center",
        Alignment::None => "",
    }
}

/// `render_cell(cell, align, context)` renders cell as html,
/// and use `align` to 
fn render_cell<'a> (_context: &RenderContext, content: Html, align: &'a Alignment) -> Html{
    html!{ 
        <td style={align_string(align)}>
            {content}
        </td>
    }
}

