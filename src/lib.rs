use rust_web_markdown::{
    render_markdown, ElementAttributes, HtmlElement, MarkdownProps, Context,
    CowStr, 
};

use core::ops::Range;

pub use rust_web_markdown::{
    LinkDescription, MdComponentProps, Options,
};

use yew::prelude::{
    function_component, html, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use web_sys::{window, MouseEvent};

use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct MarkdownMouseEvent {
    /// the original mouse event triggered when a text element was clicked on
    pub mouse_event: MouseEvent,

    /// the corresponding range in the markdown source, as a slice of [`u8`][u8]
    pub position: Range<usize>,

    // TODO: add a clonable tag for the type of the element
    // pub tag: pulldown_cmark::Tag<'a>,
}

impl<'a> Context<'a, 'static> for &'a Props {
    type View = Html;

    type HtmlCallback<T: 'static> = Callback<T, Html>;

    type Handler<T: 'static> = Callback<T>;

    type Setter<T: 'static> = UseStateHandle<T>;

    type MouseEvent = MouseEvent;

    fn props(self) -> MarkdownProps<'a, 'static, Self> {
        let Props {
            render_links,
            theme,
            wikilinks,
            hard_line_breaks,
            parse_options,
            components,
            frontmatter,
            ..
        } = self;

        MarkdownProps {
            render_links: render_links.as_ref(),
            theme: theme.as_deref(),
            wikilinks: *wikilinks,
            hard_line_breaks: *hard_line_breaks,
            parse_options: parse_options.as_ref(),
            components: &components,
            frontmatter: frontmatter.as_ref(),
        }
    }

    fn set<T>(self, setter: &UseStateHandle<T>, value: T) {
        setter.set(value)
    }

    #[cfg(feature="debug")]
    fn send_debug_info(self, info: Vec<String>) {
        if let Some(sender) = &self.send_debug_info {
            sender.emit(info)
        }
    }

    fn el_with_attributes(
        self,
        e: HtmlElement,
        inside: Self::View,
        attributes: ElementAttributes<Callback<MouseEvent>>,
    ) -> Self::View {
        let style = attributes.style.map(|x| x.to_string());
        let classes: Vec<_> = attributes.classes.iter().map(|x| x.to_string()).collect();
        let on_click = attributes.on_click;

        match e {
            HtmlElement::Div => {
                html! {<div style={style} onclick={on_click} class={classes}>{inside}</div>}
            }
            HtmlElement::Span => {
                html! {<span style={style} onclick={on_click} class={classes}>{inside}</span>}
            }
            HtmlElement::Paragraph => {
                html! {<p  style={style} onclick={on_click} class={classes}>{inside}</p>}
            }
            HtmlElement::Ul => {
                html! {<ul  style={style} onclick={on_click} class={classes}>{inside}</ul>}
            }
            HtmlElement::Ol(start) => {
                html! {<ol start={start.to_string()}  style={style} onclick={on_click} class={classes}>{inside}</ol>}
            }
            HtmlElement::Li => {
                html! {<li  style={style} onclick={on_click} class={classes}>{inside}</li>}
            }
            HtmlElement::BlockQuote => {
                html! {<blockquote  style={style} onclick={on_click} class={classes}>{inside}</blockquote>}
            }
            HtmlElement::Heading(1) => {
                html! {<h1  style={style} onclick={on_click} class={classes}>{inside}</h1>}
            }
            HtmlElement::Heading(2) => {
                html! {<h2  style={style} onclick={on_click} class={classes}>{inside}</h2>}
            }
            HtmlElement::Heading(3) => {
                html! {<h3  style={style} onclick={on_click} class={classes}>{inside}</h3>}
            }
            HtmlElement::Heading(4) => {
                html! {<h4  style={style} onclick={on_click} class={classes}>{inside}</h4>}
            }
            HtmlElement::Heading(5) => {
                html! {<h5  style={style} onclick={on_click} class={classes}>{inside}</h5>}
            }
            HtmlElement::Heading(6) => {
                html! {<h6  style={style} onclick={on_click} class={classes}>{inside}</h6>}
            }
            HtmlElement::Heading(_) => panic!(),
            HtmlElement::Table => {
                html! {<table  style={style} onclick={on_click} class={classes}>{inside}</table>}
            }
            HtmlElement::Thead => {
                html! {<thead  style={style} onclick={on_click} class={classes}>{inside}</thead>}
            }
            HtmlElement::Trow => {
                html! {<tr  style={style} onclick={on_click} class={classes}>{inside}</tr>}
            }
            HtmlElement::Tcell => {
                html! {<td  style={style} onclick={on_click} class={classes}>{inside}</td>}
            }
            HtmlElement::Italics => {
                html! {<i  style={style} onclick={on_click} class={classes}>{inside}</i>}
            }
            HtmlElement::Bold => {
                html! {<b  style={style} onclick={on_click} class={classes}>{inside}</b>}
            }
            HtmlElement::StrikeThrough => {
                html! {<s  style={style} onclick={on_click} class={classes}>{inside}</s>}
            }
            HtmlElement::Pre => {
                html! {<pre  style={style} onclick={on_click} class={classes}>{inside}</pre>}
            }
            HtmlElement::Code => {
                html! {<code  style={style} onclick={on_click} class={classes}>{inside}</code>}
            }
        }
    }

    fn el_span_with_inner_html(self, inner_html: String, attributes: ElementAttributes<Callback<MouseEvent>>) -> Self::View {
        let style = attributes.style.map(|x| x.to_string());
        let classes: Vec<_> = attributes.classes.iter().map(|x| x.to_string()).collect();
        let onclick = attributes.on_click;

        html! {
            <span style={style} onclick={onclick} class={classes}>
                {Html::from_html_unchecked(inner_html.into())}
            </span>
        }
    }

    fn el_hr(self, attributes: ElementAttributes<Callback<MouseEvent>>) -> Self::View {
        let style = attributes.style.map(|x| x.to_string());
        let classes: Vec<_> = attributes.classes.iter().map(|x| x.to_string()).collect();
        let on_click = attributes.on_click;
        html! {<hr  style={style} onclick={on_click} class={classes}/>}
    }

    fn el_br(self) -> Self::View {
        html! {<br/>}
    }

    fn el_fragment(self, children: Vec<Self::View>) -> Self::View {
        children.into_iter().collect()
    }

    fn el_a(self, children: Self::View, href: String) -> Self::View {
        html! {<a href={href.to_string()}>{children}</a>}
    }

    fn el_img(self, src: String, alt: String) -> Self::View {
        html! {<img src={src} alt={alt}/>}
    }

    fn el_text(self, text: CowStr<'a>) -> Self::View {
        html! {text}
    }

    fn mount_dynamic_link(self, rel: &str, href: &str, integrity: &str, crossorigin: &str) {
        let document = window().unwrap().document().unwrap();

        let link = document
            .create_element("link")
            .unwrap();

        link.set_attribute("rel", rel).unwrap();
        link.set_attribute("href", href).unwrap();
        link.set_attribute("integrity", integrity).unwrap();
        link.set_attribute("crossorigin", crossorigin).unwrap();

        document.head()
            .unwrap()
            .append_child(&link).unwrap();
    }

    fn el_input_checkbox(self, checked: bool, attributes: ElementAttributes<Callback<MouseEvent>>) -> Self::View {
        let style = attributes.style.map(|x| x.to_string());
        let classes: Vec<_> = attributes.classes.iter().map(|x| x.to_string()).collect();
        let on_click = attributes.on_click;
        html! {
            <input type="checkbox" checked={checked}
                onclick={on_click}
                class={classes}
                style={style}
            />
        }
    }

    fn call_handler<T: 'static>(callback: &Self::Handler<T>, input: T) {
        callback.emit(input)
    }

    fn call_html_callback<T: 'static>(callback: &Self::HtmlCallback<T>, input: T) -> Self::View {
        callback.emit(input)
    }

    fn make_handler<T: 'static, F: Fn(T) + 'static>(
        self,
        f: F,
    ) -> Self::Handler<T> {
        Callback::from(f)
    }

    fn make_md_handler(self, position: Range<usize>, stop_propagation: bool) -> Self::Handler<MouseEvent> {
        match &self.onclick {
            Some(f) => {
                let f = f.clone();
                Callback::from(move |e: MouseEvent| {
                    if stop_propagation {
                        e.stop_propagation()
                    }
                    let report = MarkdownMouseEvent {
                        mouse_event: e,
                        position: position.clone(),
                    };
                    f.emit(report)
                    }
                )
            },
            None => Callback::noop(),
        }
    }

}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub src: AttrValue,

    pub onclick: Option<Callback<MarkdownMouseEvent, ()>>,

    pub render_links: Option<Callback<LinkDescription<Html>, Html>>,

    pub theme: Option<String>,

    #[prop_or(false)]
    pub wikilinks: bool,

    #[prop_or(false)]
    pub hard_line_breaks: bool,

    pub parse_options: Option<Options>,

    #[prop_or_default]
    pub components: HashMap<String, Callback<MdComponentProps<Html>, Html>>,

    pub frontmatter: Option<UseStateHandle<String>>,

    pub send_debug_info: Option<Callback<Vec<String>>>,
}

#[function_component]
pub fn Markdown(props: &Props) -> Html {
    render_markdown(props, &props.src)
}
