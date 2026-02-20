use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use reqwest::Client;

use crate::api::HackerNewsGateway;
use crate::models::story::{HackerNewsStoryType, StoryItem};
use crate::types::HnResult;
use crate::utils::constants::HACKER_NEWS_API_BASE_URL;

/// Hacker News API versions
pub enum ApiVersion {
    V0,
}

/// Client for interacting with the Hacker News API
pub struct HackerNewsClient {
    client: Client,
    base_url: String,
}

#[async_trait]
impl HackerNewsGateway for HackerNewsClient {
    async fn fetch_stories(
        &self,
        story_type: &HackerNewsStoryType,
        limit: u16,
    ) -> HnResult<Vec<StoryItem>> {
        let story_ids: Vec<u64> = self
            .get_story_ids(story_type.parameter_name().to_string(), limit)
            .await?;
        let story_items: Vec<StoryItem> = self.get_story_items(story_ids).await?;

        return Ok(story_items);
    }
}

impl HackerNewsClient {
    pub fn new(version: Option<ApiVersion>) -> Self {
        // Get version path
        let api_version: ApiVersion = version.unwrap_or(ApiVersion::V0);
        let api_version_as_path: &str = match api_version {
            ApiVersion::V0 => "v0",
        };

        return Self {
            client: Client::new(),
            base_url: format!("{}/{}/", HACKER_NEWS_API_BASE_URL, api_version_as_path),
        };
    }

    /// Get story IDs for a given story type and limit
    async fn get_story_ids(&self, story_type: String, limit: u16) -> HnResult<Vec<u64>> {
        let url: String = format!(
            "{}{}.json?orderBy=\"$key\"&limitToFirst={}",
            self.base_url, story_type, limit
        );

        let result: Vec<u64> = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(result);
    }

    /// Get story items for a list of story IDs
    async fn get_story_items(&self, ids: Vec<u64>) -> HnResult<Vec<StoryItem>> {
        let client: Client = self.client.clone();
        let base_url: String = self.base_url.clone();

        let result: Vec<StoryItem> = stream::iter(ids)
            .map(|id: u64| {
                let client = client.clone();
                let url = format!("{base_url}item/{id}.json");
                async move { client.get(url).send().await?.json::<StoryItem>().await }
            })
            .buffer_unordered(5) // Concurrency limit
            .filter_map(|res| async { res.ok() }) // Skip failed requests or null items
            .collect::<Vec<_>>()
            .await;

        return Ok(result);
    }
}
