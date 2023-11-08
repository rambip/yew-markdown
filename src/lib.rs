use yew::{prelude::*, virtual_dom::AttrValue};
use markdown::mdast;

mod parser;
use parser::{parse, new_parse_options};

mod render;
use render::{RenderContext, render_node};
pub use render::LinkProps;

mod mouse_event;
pub use mouse_event::MarkdownMouseEvent;

pub use syntect;
pub use markdown::Constructs;
pub use markdown::unist::Point;


/// the markdown component
pub struct Markdown {
    ast: mdast::Node,
    render_context: RenderContext,
    parse_options: markdown::ParseOptions,
}


/// Properties for `Markdown`
/// `src` is the raw markdown
/// other properties:
/// `onclick`, `theme_name`, `caching`, `wikilinks`,
/// `render_link`, `onclick`
#[derive(PartialEq, Properties, Debug)]
pub struct Props {
    /// the markdown content, as raw text
    pub src: AttrValue,

    /// the constructs enabled for parsing. This will probably evolve in the future
    pub constructs: Option<Constructs>,

    /// the theme for syntax highlighting. 
    /// Please use something that `syntect` knows
    pub theme_name: Option<String>,

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

        let parse_options = new_parse_options(ctx.props().constructs.clone());

        let render_context = RenderContext::new(ctx.props().theme_name.clone(),
                                                ctx.props().onclick.clone(),
                                                ctx.props().render_link.clone()
        );


        Self {
            ast: parse(&ctx.props().src, &parse_options, ctx.props().wikilinks),
            render_context,
            parse_options,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div style="width:100%">
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/katex.min.css" integrity="sha384-3UiQGuEI4TTMaFmGIZumfRPtfKQ3trwQE2JgosJxCnGmQpL/lJdjpcHkaaFwHlcI" crossorigin="anonymous"/>
                { render_node(&self.ast, &self.render_context)}
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Props) -> bool {
        self.ast = parse(&ctx.props().src, &self.parse_options, ctx.props().wikilinks);
        true
    }
}
