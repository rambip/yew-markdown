use yew::prelude::*;
use yew_markdown::{Markdown, MdComponentProps};

use std::collections::HashMap;


static MARKDOWN_SOURCE: &str = r#"
## Here is a counter:
<Counter initial="5"/>

## Here is a Box:
<box>

**I am in a blue box !**

</box>
"#;

#[derive(PartialEq, Properties)]
struct CounterProps {
    md_props: MdComponentProps
}

#[function_component]
fn Counter(props: &CounterProps) -> Html {
    let initial: i32 = props.md_props.attributes.iter()
        .find(|(name, _)| name=="initial")
        .and_then(|(_, value)| value.parse().ok())
        .unwrap_or(0);

    let count = use_state(move || initial);

    let increment = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };
    let decrement = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };

    html!{
        <>
            <button onclick={decrement}>{"-"}</button>
            {*count}
            <button onclick={increment}>{"+"}</button>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct BoxProps {
    md_props: MdComponentProps,
}

#[function_component]
fn BoxComponent(props: &BoxProps) -> Html {
    html!{
        <div style="border: 2px solid blue">
            {props.md_props.children.clone()}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let components = HashMap::from([
        ("Counter".to_string(), Callback::from(|p| html!{<Counter md_props={p}/>})),
        ("box".to_string(), Callback::from(|p| html!{<BoxComponent md_props={p}/>}))
    ]);

    html!{
        <div>
            <h1>{"The source"}</h1>
            <pre>{MARKDOWN_SOURCE}</pre>

            <h1>{"The result"}</h1>
            <Markdown src={MARKDOWN_SOURCE} components={components}/>
        </div>
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
