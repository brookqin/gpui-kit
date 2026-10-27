mod document;
pub mod editor_host;
mod editor_seam;
mod format;
mod inline;
mod inline_element;
mod inline_flow;
mod inline_object;
#[cfg(test)]
mod inline_virtual_tests;
mod markdown_ext;
mod node;
mod range_highlight;
pub(crate) mod selection;
mod selection_adapter;
mod state;
mod stream_fade;
mod style;
mod text_view;
mod utils;

use gpui::{App, ElementId, SharedString};
pub use inline_element::*;
pub use markdown_ext::*;
pub use node::{CodeBlock, TableData};
pub use range_highlight::{RangeHighlight, RangeHighlightError, RenderedText};
pub use state::*;
pub use stream_fade::TextViewMotion;
pub use style::*;
pub use text_view::*;

pub(crate) fn init(cx: &mut App) {
    inline::init(cx);
    state::init(cx);
}

/// Create a new markdown text view with code location as id.
#[track_caller]
pub fn markdown(source: impl Into<SharedString>) -> TextView {
    let id: ElementId = ElementId::CodeLocation(*std::panic::Location::caller());
    TextView::markdown(id, source)
}

/// Create a new html text view with code location as id.
#[track_caller]
pub fn html(source: impl Into<SharedString>) -> TextView {
    let id: ElementId = ElementId::CodeLocation(*std::panic::Location::caller());
    TextView::html(id, source)
}

pub use crate::text_value::Text;
