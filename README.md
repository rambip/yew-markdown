# Goal
creating a simple library to render markdown with yew.
The best rust libraries are involved !

# Usage
To use katex, don't forget to add this stylesheet in your html:

If you use this library with trunk, add this to your `index.html`:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/katex.min.css" integrity="sha384-3UiQGuEI4TTMaFmGIZumfRPtfKQ3trwQE2JgosJxCnGmQpL/lJdjpcHkaaFwHlcI" crossorigin="anonymous">
```
Otherwise, math will not be rendered properly

Then, you can use it as a simple yew component

```rust
use yew_markdown::Markdown
html!{
    <Markdown src={"# Markdown power !"}/>
}
```

# Examples
Take a look at the different examples !
You just need trunk and a web-browser to test them.

## Showcase
the example is included in `./examples/showcase`

Here is an illustration:
![](./img/showcase.png)

## Performance
These are just some random tests to see the possible performance issues

## Editor
Of course, an example of a basic markdown editor is implemented to show whan is currently supported


# ROADMAP
- implement tables with right/left/center align
- implement note reference and image reference
- disable unused features from used crates
- publish as a crate as soon as it is stable
