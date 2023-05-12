use web_sys::MouseEvent;
use yew::prelude::*;
pub use markdown::unist::Point;

/// `mouse_event` -> the original mouse event triggered when a text element was clicked on
/// `start_position: Point` -> the corresponding starting position in the markdown source
/// `end_position: Point` -> the corresponding ending position in the markdown source
#[derive(Clone, Debug)]
pub struct MarkdownMouseEvent {
    pub mouse_event: MouseEvent,
    pub start_position: Point,
    pub end_position: Point,
}

/// `make_markdown_mouse_event_callback(onclick, position)` 
/// composes the callback `onclick` with a converter
/// to get a usable callback for the user
pub fn make_callback(onclick: &Option<Callback<MarkdownMouseEvent>>, position: &Option<markdown::unist::Position>) 
    -> Callback<MouseEvent> {
    let position = position.clone().expect("unable to know from which position the markdown tree was build");
    let onclick = onclick.clone();
    match onclick {
        Some(callback) =>{
            Callback::from(move |x| {
                let click_event = MarkdownMouseEvent {
                    mouse_event: x,
                    start_position: position.start.clone(),
                    end_position: position.end.clone(),
                };
                callback.emit(click_event)
            })
        },
        None => Callback::from(|_| ())
    }
}
