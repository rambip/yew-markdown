use stylist::{Style, StyleSource, style};
use yew::AttrValue;

fn get_style() -> Style {
    style!(r#"
         /* for the markdown container */
         & {
             width: 100%
         }

         blockquote{ 
             margin: 10px; border-left : 5px solid grey; padding:5px
         } 

         ul {
             list-style-type:disc
         }

         table {
             border: 1px solid black; border-collapse: collapse
         }

         td {
             border: 1px solid grey; padding: 5px
         }

         tr:first-child {
             background-color: #eee; font-weight: bold; 
         }

        div.math-flow {
            display: table; margin: 0 auto
        }

        span.markdown-error {
            background-color: red;
        }
        "#
    ).expect("unable to mount style")
}

fn compile_css(source: Option<AttrValue>) -> Option<Style> {
    let source: StyleSource = source?.to_string().try_into().unwrap();
    Some(Style::new(source).unwrap())
}

pub fn compile_css_or_default(source: Option<AttrValue>) -> Style {
    compile_css(source).unwrap_or(get_style())
}
