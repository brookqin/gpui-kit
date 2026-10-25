//! Plain text is always available; rich text parsing and views are optional.
mod value;
pub use value::Text;
#[cfg(feature = "rich-text")]
mod rich;
#[cfg(feature = "rich-text")]
pub use rich::*;
