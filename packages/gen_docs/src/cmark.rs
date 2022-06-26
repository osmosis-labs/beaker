use std::{borrow::Borrow, fmt};

use pulldown_cmark::Event;
use pulldown_cmark_to_cmark::{cmark_with_options, Options, State};

pub fn custom_cmark<'a, I, E, F>(events: I, mut formatter: F) -> Result<State<'static>, fmt::Error>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    F: fmt::Write,
{
    cmark_with_options(
        events,
        &mut formatter,
        Options {
            newlines_after_headline: 2,
            newlines_after_paragraph: 2,
            newlines_after_codeblock: 2,
            newlines_after_table: 2,
            newlines_after_rule: 2,
            newlines_after_list: 2,
            newlines_after_blockquote: 2,
            newlines_after_rest: 1,
            code_block_token_count: 3,
            code_block_token: '`',
            list_token: '*',
            emphasis_token: '*',
            strong_token: "**",
        },
    )
}
