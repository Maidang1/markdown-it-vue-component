use markdown_it::MarkdownIt;

use markdown_it_plugin_vue_component::{html_block_rule, html_inline_rule};

fn main() {
    let content = r#"\
<!-- @ shorthand is supported -->
<Foo @click="onClick" />

<!-- multi-line syntax won't be wrapped with <p> -->
<Foo
  class="foo"
  :bar="bar"
/>"#;
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    html_block_rule::add(parser);
    html_inline_rule::add(parser);
    let result = parser.parse(content).render();
    println!("{}", result);
}
