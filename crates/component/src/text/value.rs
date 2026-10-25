#[cfg(feature = "rich-text")]
use super::{TextView, TextViewStyle};
use gpui::{App, IntoElement, RenderOnce, SharedString, Window};

/// Either a plain string or a rich [`TextView`].
#[derive(IntoElement, Clone)]
pub enum Text {
    String(SharedString),
    #[cfg(feature = "rich-text")]
    TextView(Box<TextView>),
}
impl From<SharedString> for Text {
    fn from(value: SharedString) -> Self {
        Self::String(value)
    }
}
impl From<String> for Text {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}
impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self::String(value.to_string().into())
    }
}
#[cfg(feature = "rich-text")]
impl From<TextView> for Text {
    fn from(value: TextView) -> Self {
        Self::TextView(Box::new(value))
    }
}
impl Text {
    /// Sets the style for the [`TextView`]. Does nothing for a plain string.
    #[cfg(feature = "rich-text")]
    pub fn style(self, style: TextViewStyle) -> Self {
        match self {
            Self::String(value) => Self::String(value),
            #[cfg(feature = "rich-text")]
            Self::TextView(view) => Self::TextView(Box::new(view.style(style))),
        }
    }
    pub(crate) fn get_text(&self, _cx: &App) -> SharedString {
        match self {
            Self::String(value) => value.clone(),
            #[cfg(feature = "rich-text")]
            Self::TextView(view) => view.plain_text(_cx),
        }
    }
}
impl RenderOnce for Text {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self {
            Self::String(value) => value.into_any_element(),
            #[cfg(feature = "rich-text")]
            Self::TextView(view) => view.into_any_element(),
        }
    }
}
