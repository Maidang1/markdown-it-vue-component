use markdown_it::{
    parser::inline::{InlineRule, InlineState, TextSpecial},
    plugins::cmark::inline::autolink::Autolink,
};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

use crate::html_re::*;

fn is_letter(ch: u32) -> bool {
    let lc = ch | 0x20;
    lc >= 0x61 /* a */ && lc <= 0x7a /* z */
}

const CRAB_CLAW: &str = r#"(\/)"#;

#[derive(Debug)]
// This is a structure that represents your custom Node in AST.
pub struct InlineFerris;

// This defines how your custom node should be rendered.
impl NodeValue for InlineFerris {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        // `node.attrs` are custom attributes added by other plugins
        // (for example, source mapping information)
        let mut attrs = node.attrs.clone();

        // add a custom class attribute
        attrs.push(("class", "ferris-inline".into()));

        fmt.open("span", &attrs);
        fmt.text("🦀");
        fmt.close("span");
    }
}

// This is an extension for the inline subparser.
struct FerrisInlineScanner;

impl InlineRule for FerrisInlineScanner {
    // This is a character that starts your custom structure
    // (other characters may get skipped over).
    const MARKER: char = '(';

    // This is a custom function that will be invoked on every character
    // in an inline context.
    //
    // It should look for `state.src` exactly at position `state.pos`
    // and report if your custom structure appears there.
    //
    // If custom structure is found, it:
    //  - creates a new `Node` in AST
    //  - returns length of it
    //
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let input = &state.src[state.pos..state.pos_max]; // look for stuff at state.pos
        if !input.starts_with(CRAB_CLAW) {
            return None;
        } // return None if it's not found

        // return new node and length of this structure
        Some((Node::new(InlineFerris), CRAB_CLAW.len()))
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    md.inline.add_rule::<FerrisInlineScanner>();
}

struct HtmlInlineRule;

impl InlineRule for HtmlInlineRule {
    const MARKER: char = '<';

    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let max = state.pos_max;

        if state.src.chars().nth(state.pos).map(|c| c as u32) != Some(0x3c) || state.pos + 2 >= max
        {
            return None;
        }

        let ch = state.src.chars().nth(state.pos + 1).unwrap() as u32;

        if ch != 0x21 /* ! */ &&
          ch != 0x3f /* ? */ &&
          ch != 0x2f /* / */ &&
          !is_letter(ch)
        {
            return None;
        }

        if let Some(mat) = html_tag_re.find(&state.src[state.pos..]) {
            state.pos += mat.end();
        } else {
            return None;
        }
        return None;
        // if &state.src.chars().
    }
}
