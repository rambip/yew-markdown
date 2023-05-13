use yew::{prelude::*, virtual_dom::AttrValue};
use markdown::mdast;
use std::collections::HashMap;
// use log::info;

use stylist::Style;

mod parser;
use parser::{parse, new_parse_options};

mod render;
use render::{RenderContext, render_node};
pub use render::LinkProps;

mod style;

mod mouse_event;
pub use mouse_event::MarkdownMouseEvent;

pub use syntect;
pub use markdown::Constructs;
pub use markdown::unist::Point;


/// the markdown component
pub struct Markdown {
    ast: mdast::Node,
    cached_ast: HashMap<AttrValue, mdast::Node>,
    render_context: RenderContext,
    parse_options: markdown::ParseOptions,
    style: Style,
}


/// Properties for `Markdown`
/// `src` -> the raw markdown content to render
/// `constructs` -> the markdown 
/// `onclick`: callback executed when a portion of the markdown document  is clicked
/// `css`: the css of the component. By default there is some minimal styling 
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
    pub css: Option<AttrValue>,
    pub theme_name: Option<String>,
    pub render_link: Option<Callback<LinkProps, Html>>,

    #[prop_or(false)]
    pub caching: bool,

    #[prop_or(false)]
    pub wikilinks: bool,
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

        let style = style::compile_css_or_default(ctx.props().css.clone());


        Self {
            cached_ast: HashMap::new(),
            ast: parse(&ctx.props().src, &parse_options, ctx.props().wikilinks),
            render_context,
            style,
            parse_options,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class={self.style.get_class_name().to_string()}>
                { render_node(&self.ast, &self.render_context)}
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Props) -> bool {
        let ast = parse(&ctx.props().src, &self.parse_options, ctx.props().wikilinks);
        let old_ast = std::mem::replace(&mut self.ast, ast);
        if ctx.props().caching {
            self.cached_ast.insert(old_props.src.clone(), old_ast);
        }
        true
    }
}
