use crate::css3::node::{Node, NodeType};
use crate::css3::{Css3, Error};

impl Css3<'_> {
    pub fn parse_selector_list(&mut self) -> Result<Node, Error> {
        log::trace!("parse_selector_list");

        let loc = self.tokenizer.current_location().clone();

        let mut selectors = vec![];

        while !self.tokenizer.eof() {
            let selector = self.parse_selector()?;
            selectors.push(selector);

            let t = self.consume_any()?;
            if !t.is_comma() {
                self.tokenizer.reconsume();
                break;
            }
        }

        Ok(Node::new(NodeType::SelectorList{ selectors }, loc))
    }
}
