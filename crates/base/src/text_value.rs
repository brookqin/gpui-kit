#[cfg(feature = "rich-text")]
use crate::text::{TextView, TextViewStyle};
use gpui::{App, IntoElement, RenderOnce, SharedString, Window};

#[derive(IntoElement, Clone)]
pub enum Text {
    String(SharedString),
    #[cfg(feature = "rich-text")]
    TextView(Box<TextView>),
}

impl From<SharedString> for Text {
    fn from(s: SharedString) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::String(SharedString::from(s.to_string()))
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}

#[cfg(feature = "rich-text")]
impl From<TextView> for Text {
    fn from(e: TextView) -> Self {
        Self::TextView(Box::new(e))
    }
}

impl Text {
    /// Set the style for [`TextView`].
    ///
    /// Do nothing if this is `String`.
    #[cfg(feature = "rich-text")]
    pub fn style(self, style: TextViewStyle) -> Self {
        match self {
            Self::String(s) => Self::String(s),
            #[cfg(feature = "rich-text")]
            Self::TextView(e) => Self::TextView(Box::new(e.style(style))),
        }
    }

    /// Get the text content.
    #[doc(hidden)]
    pub fn get_text(&self, _cx: &App) -> SharedString {
        match self {
            Self::String(s) => s.clone(),
            #[cfg(feature = "rich-text")]
            Self::TextView(view) => {
                if let Some(state) = &view.state {
                    state.read(_cx).source()
                } else {
                    SharedString::default()
                }
            }
        }
    }
}

impl RenderOnce for Text {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self {
            Self::String(s) => s.into_any_element(),
            #[cfg(feature = "rich-text")]
            Self::TextView(e) => e.into_any_element(),
        }
    }
}
