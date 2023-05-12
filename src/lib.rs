// mod katex;
use yew::{prelude::*, virtual_dom::AttrValue};
use markdown::mdast;
use std::collections::HashMap;
// use log::info;

use stylist::Style;

mod render;
use render::{RenderContext, render_node};
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
            gfm_table: true,

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

        let render_context = RenderContext::new(ctx.props().theme_name.clone(),
                                                ctx.props().onclick.clone());

        let style = style::compile_css_or_default(ctx.props().css.clone());


        Self {
            cached_ast: HashMap::new(),
            ast: markdown::to_mdast(&ctx.props().src, &parse_options).unwrap(),
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
