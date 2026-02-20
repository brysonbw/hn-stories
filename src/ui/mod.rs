pub mod terminal;

use async_trait::async_trait;

use crate::{
    models::story::{HackerNewsStoryType, StoryItem},
    types::HnResult,
};

/// An abstraction for user interface operations
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserInterface {
    /// Renders list of Hacker News stories and handles detail view
    async fn render_stories_list(
        &self,
        stories: Vec<StoryItem>,
        story_type: &HackerNewsStoryType,
    ) -> HnResult<()>;
}
