pub use markdown::unist::{Point, Position};

use core::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Pipe,
    RBra,
    LBra,
    RRBra,
    LLBra,
    Word(String),
    NewLine,
}

use Token::*;

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Pipe => "|".into(),
            RBra => "]".into(),
            LBra => "[".into(),
            RRBra => "]]".into(),
            LLBra => "[[".into(),
            Word(s) => s.clone(),
            NewLine => "\n".into(),
        }
    }
}

enum State {
    Default,
    AfterPipe(Point),
    AfterOpen1(Point),
    AfterOpen2(Position),
    AfterOpen3(Point),
    AfterClose1(Point),
    AfterClose2(Position),
    AfterClose3(Point),
    AfterSymbol(Position, String),
    AfterReturn(Point),
}

impl Default for State {
    fn default() -> Self {
        State::Default
    }
}

fn pos2(start: Point, end: Point) -> Position {
    Position{
        start, end
    }
}

fn pos1(p: Point) -> Position {
    Position {
        start: p.clone(),
        end: p,
    }
}

impl State {
    fn finalize(self: State) -> Option<(Token, Position)> {
        use State::*;

        Some(match self {
            AfterPipe(p) => (Pipe, pos1(p)),
            AfterOpen1(p) => (LBra, pos1(p)),
            AfterOpen2(x) => (LLBra, x),
            AfterOpen3(p) => (LBra, pos1(p)),
            AfterClose1(p) => (RBra, pos1(p)),
            AfterClose2(x) => (RRBra, x),
            AfterClose3(p) => (RBra, pos1(p)),
            AfterSymbol(x, s) => (Word(s), x),
            AfterReturn(p) => (NewLine, pos1(p)),
            Default => return None,
        })
    }
}


fn advance(point: &mut Point, c: char) {
    if c == '\n' {
        point.line += 1;
        point.column = 1;
    }
    else {
        point.column += c.len_utf8();
    }
    point.offset += c.len_utf8();
}

pub struct TokenStream<'a> {
    source: core::str::Chars<'a>,
    cursor: Point,
    state: State,
}

impl<'a> TokenStream<'a> {
    pub fn new_at(source: &'a str, point: Point) -> TokenStream<'a> {
        TokenStream {
            source: source.chars(),
            cursor: point.clone(),
            state: State::Default,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = (Token, Position);

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;

        for c in self.source.by_ref() {

            let cursor = self.cursor.clone();
            let transition = |old_state| match (c, old_state) {
                ('\r', x) => (x, None),
                ('\n', x) => ( AfterReturn(cursor), x.finalize()),
                ('[', AfterOpen1(p)) =>  (AfterOpen2(pos2(p, cursor)), None),
                ('[', AfterOpen2(p)) =>  (AfterOpen3(cursor), AfterOpen2(p).finalize()),
                ('[', AfterOpen3(p)) =>  (AfterOpen3(cursor), AfterOpen3(p).finalize()),
                ('[', x) =>              (AfterOpen1(cursor), x.finalize()),
                (']', AfterClose1(p)) => (AfterClose2(pos2(p, cursor)), None),
                (']', AfterClose2(p)) => (AfterClose3(cursor), AfterClose2(p).finalize()),
                (']', AfterClose3(p)) => (AfterClose3(cursor), AfterClose3(p).finalize()),
                (']', x) =>              (AfterClose1(cursor), x.finalize()),
                ('|', x) =>              (AfterPipe(cursor), x.finalize()),
                (c, AfterSymbol(p, mut s)) => {
                    s.push(c);
                    (AfterSymbol(pos2(p.start, cursor), s), None)
                }
                (c, x) => (AfterSymbol(pos2(cursor.clone(), cursor), c.into()), x.finalize())

            };
            let state = std::mem::take(&mut self.state);
            let (state, item) = transition(state);
            self.state = state;
            advance(&mut self.cursor, c);

            if item.is_some() {return item}
        }
        (std::mem::take(&mut self.state)).finalize()
    }
}

#[cfg(test)]
mod tests {
    use wasm_test::*;
    use super::*;

    #[wasm_test]
    fn test_stream(){
        let source = "[abc] [[ d e]]\nb";
        let stream: Vec<(Token, Position)> = TokenStream::new_at(source, Point::new(1,1,0))
            .collect();
        println!("{stream:?}");
        assert_eq!(stream, 
                   vec![
                       (LBra,Position::new(1,1,0, 1,1,0)), 
                       (Word("abc".into()), Position::new(1,2,1, 1,4,3)),
                       (RBra,Position::new(1,5,4, 1,5,4)), 
                       (Word(" ".into()), Position::new(1,6,5, 1,6,5)),
                       (LLBra, Position::new(1,7,6, 1,8,7)), 
                       (Word(" d e".into()), Position::new(1,9,8, 1,12, 11)),
                       (RRBra, Position::new(1,13,12, 1,14,13)),
                       (NewLine, Position::new(1,15,14, 1,15,14)),
                       (Word("b".into()), Position::new(2,1,15, 2,1,15)),
                   ]
        );
    }

    #[wasm_test]
    fn test_stream_double_bracket(){
        let source = "[[[";
        let stream: Vec<(Token, Position)> 
            = TokenStream::new_at(source, Point::new(1,1,0)).collect();

        println!("{stream:?}");
        assert_eq!(stream, 
                   vec![
                       (LLBra, Position::new(1,1,0, 1,2,1)), 
                       (LBra, Position::new(1,3,2, 1,3,2)), 
                   ]
        );
    }
}
