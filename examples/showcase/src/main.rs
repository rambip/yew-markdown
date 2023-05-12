use yew::prelude::*;
use yew_markdown::Markdown;

static MARKDOWN_SOURCE : &str = r#"
## Code
```rust
fn main() {
    println!("hello world !")
}
```

## Math
1) $1+1=2$

2) $e^{i\pi}+1=0$

3)
$$\int_0^{+\infty}\dfrac{\sin(t)}{t}\,dt=\dfrac{\sqrt{\pi}}{2}$$


## Links and images
for a markdown cheatsheet, see https://commonmark.org/help/

[the markdown engine](https://github.com/wooorm/markdown-rs)
![](https://raw.githubusercontent.com/wooorm/markdown-rs/8924580/media/logo-monochromatic.svg?sanitize=true)

## Style
| unstyled | styled    |
| :-----:  | ------    |
| bold     | **bold**  |
| italics  | *italics* |
| strike   | ~strike~  |

> Hey, I am a quote !
> - I don't like numbers
"#;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Markdown src={MARKDOWN_SOURCE}/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
