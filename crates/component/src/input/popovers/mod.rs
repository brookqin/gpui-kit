mod caret_anchored;
mod code_action_menu;
mod completion_menu;
mod diagnostic_popover;
mod hover_popover;

pub(crate) use caret_anchored::*;
pub(crate) use code_action_menu::*;
pub(crate) use completion_menu::*;
pub(crate) use diagnostic_popover::*;
pub(crate) use hover_popover::*;

use gpui::{
    App, Div, ElementId, InteractiveElement as _, SharedString, Stateful, Styled as _, Window, div,
};

use crate::ThemeStyled as _;

#[cfg(feature = "rich-text")]
use crate::ActiveTheme;
#[cfg(feature = "rich-text")]
use crate::text::{TextView, TextViewStyle};
#[cfg(feature = "rich-text")]
use gpui::{StyleRefinement, px, rems};

#[cfg(feature = "rich-text")]
pub(super) fn render_markdown(
    id: impl Into<ElementId>,
    markdown: impl Into<SharedString>,
    _: &mut Window,
    cx: &mut App,
) -> TextView {
    TextView::markdown(id, markdown)
        .style(
            TextViewStyle::default()
                .paragraph_gap(rems(0.5))
                .heading_font_size(|level, rem_size| match level {
                    1..=3 => rem_size * 1,
                    4 => rem_size * 0.9,
                    _ => rem_size * 0.8,
                })
                .code_block(
                    StyleRefinement::default()
                        .bg(cx.theme().transparent)
                        .p_0()
                        .text_size(px(11.)),
                ),
        )
        .selectable(true)
}

pub(super) fn editor_popover(id: impl Into<ElementId>, cx: &App) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .occlude()
        .popover_style(cx)
        .shadow_md()
        .text_xs()
        .p_1()
}

#[cfg(not(feature = "rich-text"))]
pub(super) fn render_markdown(
    _: impl Into<ElementId>,
    text: impl Into<SharedString>,
    _: &mut Window,
    _: &mut App,
) -> SharedString {
    text.into()
}
