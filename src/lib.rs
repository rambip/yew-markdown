use yew::{prelude::*, virtual_dom::AttrValue};

use pulldown_cmark_wikilink::ParserOffsetIter;

mod render;
use render::{RenderContext, Renderer};

mod utils;

pub use syntect;

use web_sys::MouseEvent;
use core::ops::Range;

/// `mouse_event` -> the original mouse event triggered when a text element was clicked on
/// `position` -> the range between the starting offset and the end offset
#[derive(Clone, Debug)]
pub struct MarkdownMouseEvent {
    pub mouse_event: MouseEvent,
    pub position: Range<usize>
}

/// the description of a link, used to render it with a custom callback.
/// See [pulldown_cmark::Tag::Link] for documentation
pub struct LinkProps {
    /// the url of the link
    pub url: String,

    /// the html view of the element under the link
    pub content: Html,

    /// the title of the link. 
    /// If you don't know what it is, don't worry: it is ofter empty
    pub title: String,

    /// the type of link
    pub link_type: pulldown_cmark_wikilink::LinkType,

    /// wether the link is an image
    pub image: bool,
}


/// the markdown component
pub struct Markdown {
    render_context: RenderContext,
    parse_options: pulldown_cmark_wikilink::Options,
}


/// Properties for `Markdown`
/// `src` is the raw markdown
/// other properties:
/// `onclick`, `theme`, `wikilinks`,
/// `render_link`, `onclick`
#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    /// the markdown content, as raw text
    pub src: AttrValue,

    /// the constructs enabled for parsing. This will probably evolve in the future
    pub parse_options: Option<pulldown_cmark_wikilink::Options>,

    /// the theme for syntax highlighting. 
    /// Please use something that `syntect` knows
    pub theme: Option<String>,

    #[prop_or(false)]
    /// wether you allow wikilinks.
    /// By default, [[link|alias]] will be converted to 
    /// ```
    /// LinkProps {
    ///     address = link,
    ///     alias = alias,
    ///     title = None
    /// }
    /// ```
    /// And then to a link, unless you set `render_link`
    pub wikilinks: bool,

    /// the callback used to render links. By default
    /// ```
    /// render_link = Callback::from(move |link| 
    ///     html!{<a href={link.address.clone()}>{link.alias.clone()}</a>}
    /// )
    /// ```
    pub render_link: Option<Callback<LinkProps, Html>>,

    /// a callback emmited when you click the markdown content.
    /// It takes an argument of type `MarkdownMouseEvent`
    pub onclick: Option<Callback<MarkdownMouseEvent>>,
}



impl Component for Markdown {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {

        let render_context = RenderContext::new(ctx.props().theme.clone(),
                                                ctx.props().onclick.clone(),
                                                ctx.props().render_link.clone()
        );

        let parse_options = ctx.props().parse_options
            .unwrap_or(pulldown_cmark_wikilink::Options::all());

        Self {
            render_context,
            parse_options
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ast: Vec<_> = ParserOffsetIter::new_ext(
            &ctx.props().src, 
            self.parse_options, 
            ctx.props().wikilinks
        )
        .collect();

        html!{
            <div style="width:100%">
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/katex.min.css" integrity="sha384-3UiQGuEI4TTMaFmGIZumfRPtfKQ3trwQE2JgosJxCnGmQpL/lJdjpcHkaaFwHlcI" crossorigin="anonymous"/>
                { Renderer::new(&self.render_context, &mut ast.into_iter())
                    .collect::<Html>()
                }
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Props) -> bool {
        let new_props = ctx.props();
        if new_props.theme != old_props.theme {
            self.render_context = RenderContext::new(
                ctx.props().theme.clone(), 
                ctx.props().onclick.clone(), 
                ctx.props().render_link.clone()
            );
        }
        if new_props.parse_options != old_props.parse_options {
            self.parse_options = ctx.props().parse_options
                .unwrap_or(pulldown_cmark_wikilink::Options::all());
        }

        true
    }
}
