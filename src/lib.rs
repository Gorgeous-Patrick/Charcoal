//! Charcoal, a command line dictionary
//!
//! Charcoal uses youdao dict api and google speech. Inspired by wudao-dict.

pub mod app;
pub mod word;

pub use app::builder::AppBuilder;
pub use app::cache::Cache;
pub use app::cli::{Cli, Commands};
pub use app::config::Config;
pub use app::App;
pub use word::speech::Speech;
pub use word::{Acquire, PPrint};
pub use word::{Answer, Question};
pub use word::frontend::{ExactQuery, SingleEntry};
