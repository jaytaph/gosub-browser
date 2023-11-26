use crate::css3::node::Node;
use crate::css3::tokenizer::TokenType;
use crate::css3::{Css3, Error};

impl Css3<'_> {
    pub fn parse_stylesheet(&mut self) -> Result<Node, Error> {
        log::trace!("parse_stylesheet");

        let mut children = Vec::new();

        while !self.tokenizer.eof() {
            let t = self.consume_any()?;

            match t.token_type {
                TokenType::Eof => {}
                TokenType::Whitespace => {}
                TokenType::Comment(comment) => {
                    if comment.chars().nth(2) == Some('!') {
                        children.push(Node::new_comment(comment));
                    }
                }
                TokenType::Cdo => {
                    children.push(Node::new_cdo());
                }
                TokenType::Cdc => {
                    children.push(Node::new_cdc());
                }
                TokenType::AtKeyword(_keyword) => {
                    self.tokenizer.reconsume();
                    let at_rule = self.parse_at_rule(false)?;
                    children.push(at_rule);
                }
                _ => {
                    self.tokenizer.reconsume();
                    let rule = self.parse_rule()?;
                    children.push(rule);
                }
            }
        }

        Ok(Node::new_stylesheet(children))
    }
}
