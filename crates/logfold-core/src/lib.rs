//! Fold log lines into normalized patterns, ordered by frequency.
//!
//! Lines match when their text is identical after whitespace normalization and
//! masking of timestamps, UUIDs, IPv4 addresses, hexadecimal values, and numbers.
//! Empty lines are ignored; ties are ordered by pattern text.
//!
//! ```
//! use logfold_core::{Folder, Pattern};
//!
//! let mut folder = Folder::new();
//! folder.add("user 123 logged in from 10.0.0.5");
//! folder.add("user 456 logged in from 192.168.1.9");
//!
//! assert_eq!(folder.finish(), vec![Pattern {
//!     text: "user <NUM> logged in from <IP>".into(),
//!     count: 2,
//! }]);
//! ```

mod fold;
mod mask;

pub use fold::{Folder, Pattern};
pub use mask::mask_line;
