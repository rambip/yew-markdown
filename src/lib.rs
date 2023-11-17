use rust_web_markdown::{
    render_markdown, ElementAttributes, HtmlElement, MarkdownProps, WebFramework,
};

pub use rust_web_markdown::{
    LinkDescription, MarkdownMouseEvent, MdComponentProps, Options,
};

use yew::prelude::{
    function_component, html, AttrValue, Callback, Html, Properties, UseStateHandle,
};

use web_sys::window;

use std::collections::HashMap;

#[derive(Clone)]
pub struct MarkdownContext {
    send_debug_info: Option<Callback<Vec<String>>>,
}

impl WebFramework<'static> for MarkdownContext {
    type View = Html;

    type HtmlCallback<T: 'static> = Callback<T, Html>;

    type Callback<A: 'static, B: 'static> = Callback<A, B>;

    type Setter<T> = UseStateHandle<T>;

    fn set<T>(&self, setter: &UseStateHandle<T>, value: T) {
        setter.set(value)
    }

    fn send_debug_info(&self, info: Vec<String>) {
        if let Some(sender) = &self.send_debug_info {
            sender.emit(info)
        }
    }

    fn el_with_attributes(
        &self,
        e: HtmlElement,
        inside: Self::View,
        attributes: ElementAttributes<'static, Self>,
    ) -> Self::View {
        let style = attributes.style.map(|x| x.to_string());
        let classes: Vec<_> = attributes.classes.iter().map(|x| x.to_string()).collect();
        let on_click = attributes.on_click;

        let inside = match attributes.inner_html {
            Some(i) => Html::from_html_unchecked(i.to_string().into()),
            None => inside
        };

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

    fn el_hr(&self, attributes: ElementAttributes<'static, Self>) -> Self::View {
        let style = attributes.style.map(|x| x.to_string());
        let classes: Vec<_> = attributes.classes.iter().map(|x| x.to_string()).collect();
        let on_click = attributes.on_click;
        html! {<hr  style={style} onclick={on_click} class={classes}/>}
    }

    fn el_br(&self) -> Self::View {
        html! {<br/>}
    }

    fn el_fragment(&self, children: Vec<Self::View>) -> Self::View {
        children.into_iter().collect()
    }

    fn el_a(&self, children: Self::View, href: &str) -> Self::View {
        html! {<a href={href.to_string()}>{children}</a>}
    }

    fn el_img(&self, src: &str, alt: &str) -> Self::View {
        html! {<img src={src.to_string()} alt={alt.to_string()}/>}
    }

    fn el_text(&self, text: &str) -> Self::View {
        html! {text}
    }

    fn mount_dynamic_link(&self, rel: &str, href: &str, integrity: &str, crossorigin: &str) {
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

    fn el_input_checkbox(&self, checked: bool, attributes: ElementAttributes<'static, Self>) -> Self::View {
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

    fn call_callback<A: 'static, B: 'static>(callback: &Self::Callback<A, B>, input: A) -> B {
        callback.emit(input)
    }

    fn call_html_callback<T: 'static>(callback: &Self::HtmlCallback<T>, input: T) -> Self::View {
        callback.emit(input)
    }

    fn make_callback<A: 'static, B: 'static, F: Fn(A) -> B + 'static>(
        f: F,
    ) -> Self::Callback<A, B> {
        Callback::from(f)
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub src: AttrValue,

    pub onclick: Option<Callback<MarkdownMouseEvent, ()>>,

    pub render_links: Option<Callback<LinkDescription<'static, MarkdownContext>, Html>>,

    pub theme: Option<String>,

    #[prop_or(false)]
    pub wikilinks: bool,

    #[prop_or(false)]
    pub hard_line_breaks: bool,

    pub parse_options: Option<Options>,

    #[prop_or_default]
    pub components: HashMap<String, Callback<MdComponentProps<'static, MarkdownContext>, Html>>,

    pub frontmatter: Option<UseStateHandle<String>>,

    pub send_debug_info: Option<Callback<Vec<String>>>,
}

#[function_component]
pub fn Markdown(props: &Props) -> Html {
    let Props {
        src,
        onclick,
        render_links,
        theme,
        wikilinks,
        hard_line_breaks,
        parse_options,
        components,
        frontmatter,
        send_debug_info,
    } = props;

    let props = MarkdownProps {
        on_click: onclick.as_ref(),
        render_links: render_links.as_ref(),
        theme: theme.as_deref(),
        wikilinks: *wikilinks,
        hard_line_breaks: *hard_line_breaks,
        parse_options: parse_options.as_ref(),
        components: &components,
        frontmatter: frontmatter.as_ref(),
    };
    let cx = MarkdownContext {
        send_debug_info: send_debug_info.clone(),
    };
    render_markdown(cx, src, props)
}
