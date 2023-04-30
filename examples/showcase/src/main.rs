use yew::prelude::*;
use yew_markdown::{Markdown, Constructs};

static MARKDOWN_SOURCE : &str = r#"
# Code
```rust
fn main() {
    println!("hello world !")
}
```

# Math
$1+1=2$

$$\int_0^{+\infty}\dfrac{\sin(t)}{t}\,dt=\dfrac{\sqrt{\pi}}{2}$$

# Footnote
> Hey !

"#;

#[function_component(App)]
fn app() -> Html {
    let constructs = Constructs{
        math_text: true,
        math_flow: true,
        frontmatter: true,
        ..Constructs::default()
    };
    html! {
        <Markdown source={MARKDOWN_SOURCE} constructs={constructs}/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
