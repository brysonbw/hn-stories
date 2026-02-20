use async_trait::async_trait;

use crate::models::story::{HackerNewsStoryType, StoryItem};
use crate::types::HnResult;

/// A gateway to the Hacker News API
///
/// This trait abstracts the communication logic required to fetch stories from Hacker News. By using a trait, you can easily
/// swap the production HTTP implementation with a mock during unit testing.
///
/// Due to the `Send + Sync` bounds, implementations of this trait are safe
/// to share across thread boundaries.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait HackerNewsGateway: Send + Sync {
    /// Fetch Hacker News story items
    async fn fetch_stories(
        &self,
        story_type: &HackerNewsStoryType,
        limit: u16,
    ) -> HnResult<Vec<StoryItem>>;
}
