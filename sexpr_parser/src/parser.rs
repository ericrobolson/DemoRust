use crate::error;
use crate::location::Location;
use crate::tokenizer::{Token, TokenKind};

pub type Err = error::Error<ParserErr>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErr {
    List(ListErr),
    StackUnderflow,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ListErr {
    UnclosedList,
    UnstartedList,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub ast: Ast,
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    List(Vec<Node>),
    Comment(String),
    Identifier(String),
    Number(f64),
    String(String),
}

/// A structure for parsing.
pub struct Parser {
    current_location: Location,
    nodes: Vec<Node>,
    tokens: Vec<Token>,
    state_stack: Vec<State>,
}
impl Parser {
    /// Attempts to parse the given tokens into a vec of nodes.
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, Err> {
        let mut parser = Self::new(tokens);

        while let Some(token) = parser.next_token() {
            match &token.kind {
                TokenKind::Comment(comment) => {
                    let node = Node {
                        ast: Ast::Comment(comment.clone()),
                        tokens: vec![token],
                    };
                    parser.add_node(node)?;
                }
                TokenKind::Number(n) => {
                    let node = Node {
                        ast: Ast::Number(*n),
                        tokens: vec![token],
                    };
                    parser.add_node(node)?;
                }
                TokenKind::Identifier(id) => {
                    let node = Node {
                        ast: Ast::Identifier(id.clone()),
                        tokens: vec![token],
                    };
                    parser.add_node(node)?;
                }
                TokenKind::String(string) => {
                    let node = Node {
                        ast: Ast::String(string.clone()),
                        tokens: vec![token],
                    };
                    parser.add_node(node)?;
                }
                TokenKind::Symbol('(') => {
                    parser.start_list(token);
                }
                TokenKind::Symbol(')') => {
                    parser.end_list(token)?;
                }
                t => todo!("Parse token: {:#?}", t),
            }
        }

        parser.finalize()
    }

    /// Adds the node to the list of nodes.
    fn add_node(&mut self, node: Node) -> Result<(), Err> {
        // Add to previous state if making a list
        if self.is_making_list() {
            let mut state = self.pop_list_state()?;
            state.nodes.push(node);
            self.state_stack.push(State::List(state));
        } else {
            self.nodes.push(node);
        }
        Ok(())
    }

    /// Ends a list.
    fn end_list(&mut self, token: Token) -> Result<(), Err> {
        if self.is_making_list() {
            let mut state = self.pop_list_state()?;
            state.tokens.push(token);

            let node = Node {
                ast: Ast::List(state.nodes),
                tokens: state.tokens,
            };
            self.add_node(node)?;

            Ok(())
        } else {
            Err(self.make_err(ParserErr::List(ListErr::UnstartedList)))
        }
    }

    /// Finalizes the nodes.
    fn finalize(mut self) -> Result<Vec<Node>, Err> {
        while let Ok(state) = self.pop_state() {
            let err = match state {
                State::List(state) => {
                    let mut err = self.make_err(ParserErr::List(ListErr::UnclosedList));

                    err.location = state.start;
                    Some(err)
                }
                _ => None,
            };

            match err {
                Some(e) => return Err(e),
                None => {}
            }
        }

        Ok(self.nodes)
    }

    /// Returns whether a list is being made or not.
    fn is_making_list(&self) -> bool {
        if let Some(State::List(_)) = self.peek_state() {
            true
        } else {
            false
        }
    }

    /// Creates an error.
    fn make_err(&mut self, kind: ParserErr) -> Err {
        error::Error {
            kind,
            location: self.current_location.clone(),
        }
    }

    /// Creates a new parser.
    fn new(tokens: Vec<Token>) -> Self {
        let current_location = if tokens.is_empty() == false {
            tokens[0].location.clone()
        } else {
            Location::new("".into())
        };

        Self {
            current_location,
            tokens,
            nodes: vec![],
            state_stack: vec![],
        }
    }

    /// Attempts to get the next token.
    fn next_token(&mut self) -> Option<Token> {
        if self.tokens.is_empty() {
            None
        } else {
            let token = self.tokens.remove(0);
            self.current_location = token.location.clone();

            Some(token)
        }
    }

    /// Peeks the top of the state stack.
    fn peek_state(&self) -> Option<&State> {
        if self.state_stack.is_empty() {
            None
        } else {
            Some(&self.state_stack[self.state_stack.len() - 1])
        }
    }

    /// Pops a list state.
    fn pop_list_state(&mut self) -> Result<ListState, Err> {
        match self.pop_state()? {
            State::List(state) => Ok(state),
        }
    }

    /// Pops off the top most state.
    fn pop_state(&mut self) -> Result<State, Err> {
        match self.state_stack.pop() {
            Some(state) => Ok(state),
            None => Err(self.make_err(ParserErr::StackUnderflow)),
        }
    }

    /// Starts a list.
    fn start_list(&mut self, token: Token) {
        self.state_stack.push(State::List(ListState {
            start: token.location.clone(),
            nodes: vec![],
            tokens: vec![token],
        }))
    }
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    List(ListState),
}

#[derive(Debug, Clone, PartialEq)]
struct ListState {
    start: Location,
    nodes: Vec<Node>,
    tokens: Vec<Token>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn parse_returns_empty_list() {
        let contents = "()";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![]),
            tokens: tokens.clone(),
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_single_nested_list() {
        let contents = "(())";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![Node {
                ast: Ast::List(vec![]),
                tokens: vec![tokens[1].clone(), tokens[2].clone()],
            }]),
            tokens: vec![tokens[0].clone(), tokens[3].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_identifier_comment_and_list() {
        let contents = "(
            foo
            ;; Test 2
        )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![
                Node {
                    ast: Ast::Identifier("foo".into()),
                    tokens: vec![tokens[1].clone()],
                },
                Node {
                    ast: Ast::Comment("Test 2".into()),
                    tokens: vec![tokens[2].clone()],
                },
            ]),
            tokens: vec![tokens[0].clone(), tokens[3].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_identifier_number_and_list() {
        let contents = "(
            foo
            3.14
        )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![
                Node {
                    ast: Ast::Identifier("foo".into()),
                    tokens: vec![tokens[1].clone()],
                },
                Node {
                    ast: Ast::Number(3.14),
                    tokens: vec![tokens[2].clone()],
                },
            ]),
            tokens: vec![tokens[0].clone(), tokens[3].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_negative_number() {
        let contents = "-1.22";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::Number(-1.22),
            tokens: vec![tokens[0].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_string_number_and_list() {
        let contents = "(
            \"foo\"
            3.14
        )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![
                Node {
                    ast: Ast::String("foo".into()),
                    tokens: vec![tokens[1].clone()],
                },
                Node {
                    ast: Ast::Number(3.14),
                    tokens: vec![tokens[2].clone()],
                },
            ]),
            tokens: vec![tokens[0].clone(), tokens[3].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_comment_and_list() {
        let contents = "(
            ;; Test 1
            ;; Test 2
        )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![
                Node {
                    ast: Ast::Comment("Test 1".into()),
                    tokens: vec![tokens[1].clone()],
                },
                Node {
                    ast: Ast::Comment("Test 2".into()),
                    tokens: vec![tokens[2].clone()],
                },
            ]),
            tokens: vec![tokens[0].clone(), tokens[3].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_double_nested_list() {
        let contents = "( () () )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![
                Node {
                    ast: Ast::List(vec![]),
                    tokens: vec![tokens[1].clone(), tokens[2].clone()],
                },
                Node {
                    ast: Ast::List(vec![]),
                    tokens: vec![tokens[3].clone(), tokens[4].clone()],
                },
            ]),
            tokens: vec![tokens[0].clone(), tokens[5].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_triple_nested_list() {
        let contents = "( ( () ) () )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = vec![Node {
            ast: Ast::List(vec![
                Node {
                    ast: Ast::List(vec![Node {
                        ast: Ast::List(vec![]),
                        tokens: vec![tokens[2].clone(), tokens[3].clone()],
                    }]),
                    tokens: vec![tokens[1].clone(), tokens[4].clone()],
                },
                Node {
                    ast: Ast::List(vec![]),
                    tokens: vec![tokens[5].clone(), tokens[6].clone()],
                },
            ]),
            tokens: vec![tokens[0].clone(), tokens[7].clone()],
        }];

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn parse_returns_err_on_unclosed_list() {
        let contents = "(";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = error::Error {
            kind: ParserErr::List(ListErr::UnclosedList),
            location: tokens[0].location.clone(),
        };

        assert_eq!(expected, actual.unwrap_err());
    }

    #[test]
    fn parse_returns_err_on_complex_unclosed_list() {
        let contents = "( ( ()  () )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = error::Error {
            kind: ParserErr::List(ListErr::UnclosedList),
            location: tokens[0].location.clone(),
        };

        assert_eq!(expected, actual.unwrap_err());
    }

    #[test]
    fn parse_returns_err_on_second_complex_unclosed_list() {
        let contents = "( ()) (  ( )";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = error::Error {
            kind: ParserErr::List(ListErr::UnclosedList),
            location: tokens[4].location.clone(),
        };

        assert_eq!(expected, actual.unwrap_err());
    }

    #[test]
    fn parse_returns_err_on_unstarted_list() {
        let contents = ")";
        let path: PathBuf = "derpy".into();
        let tokens = Tokenizer::tokenize(contents, path).unwrap();

        let actual = Parser::parse(tokens.clone());
        let expected = error::Error {
            kind: ParserErr::List(ListErr::UnstartedList),
            location: tokens[0].location.clone(),
        };

        assert_eq!(expected, actual.unwrap_err());
    }
}
