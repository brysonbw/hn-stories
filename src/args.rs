use chroma_print::print_warn;
use clap::Parser;

use crate::api::HackerNewsGateway;
use crate::models::story::{HackerNewsStoryType, StoryItem};
use crate::types::HnResult;
use crate::ui::UserInterface;
use crate::utils::helpers::{clear_loading, show_loading};

/// Command line arguments for interactively fetching, browsing, and opening Hacker News stories
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Story type (top, new, best, ask, show, job)
    #[arg(short = 's', long = "story", ignore_case = true, default_value = "t")]
    story: HackerNewsStoryType,

    /// The number of stories to fetch and display in the terminal UI
    #[arg(short = 'l', long = "limit", ignore_case = true, default_value = "30")]
    limit: u16,
}

impl Args {
    /// Run/execute command line arguments
    pub async fn run<G, U>(self, gateway: G, ui: U) -> HnResult<()>
    where
        G: HackerNewsGateway,
        U: UserInterface,
    {
        // Validate
        let max_limit: u16 = self.story.max_stories_limit();
        if self.limit > max_limit {
            return Err(format!(
                "Max story limit for '{}' stories is {}",
                self.story.full_name(),
                max_limit
            )
            .into());
        }

        show_loading(Some(&format!(
            "Fetching {} stories...please wait",
            self.story.full_name()
        )));

        let result: HnResult<Vec<StoryItem>> = gateway.fetch_stories(&self.story, self.limit).await;

        clear_loading();

        let stories: Vec<StoryItem> = result?;

        if stories.is_empty() {
            print_warn!("No {} stories found to display", self.story.full_name());
            return Ok(());
        }

        ui.render_stories_list(stories, &self.story).await?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::MockHackerNewsGateway;
    use crate::models::story::{HackerNewsStoryType, StoryItem};
    use crate::ui::MockUserInterface;

    const MAX_STORY_LIMIT_ERROR_SUBSTRING: &str = "Max story limit";

    #[tokio::test]
    async fn test_run() {
        // Setup mocks: gateway and ui
        let mut mock_gateway = MockHackerNewsGateway::new();
        let fake_stories: Vec<StoryItem> = vec![StoryItem {
            id: 1,
            by: Some("johndow".to_string()),
            url: Some("https://google.com".to_string()),
            score: Some(100),
            title: Some("Test Story".to_string()),
        }];

        mock_gateway
            .expect_fetch_stories()
            .times(1)
            .returning(move |_, _| Ok(fake_stories.clone()));

        let mut mock_ui = MockUserInterface::new();
        mock_ui
            .expect_render_stories_list()
            .times(1)
            .returning(|_, _| Ok(()));

        // Create args
        let args = Args {
            story: HackerNewsStoryType::T,
            limit: 1,
        };

        let result: HnResult<()> = args.run(mock_gateway, mock_ui).await;

        assert!(
            result.is_ok(),
            "The run function failed. Error: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_run_invalid_story_max_limit() {
        let story_types: [HackerNewsStoryType; 6] = [
            HackerNewsStoryType::N,
            HackerNewsStoryType::T,
            HackerNewsStoryType::B,
            HackerNewsStoryType::A,
            HackerNewsStoryType::S,
            HackerNewsStoryType::J,
        ];

        for story_type in story_types {
            let mock_gateway = MockHackerNewsGateway::new();
            let mock_ui = MockUserInterface::new();

            let args = Args {
                story: story_type,
                limit: story_type.max_stories_limit() + 1,
            };

            let result: HnResult<()> = args.run(mock_gateway, mock_ui).await;

            assert!(
                result.is_err(),
                "Test case for '{}' story should have returned an error",
                story_type.full_name()
            );

            let error_message: String = result.unwrap_err().to_string();
            assert!(
                error_message.contains(MAX_STORY_LIMIT_ERROR_SUBSTRING),
                "Test case for '{}' story failed. Expected error containing: '{}'. Got: '{}'",
                story_type.full_name(),
                MAX_STORY_LIMIT_ERROR_SUBSTRING,
                error_message
            );
        }
    }
}
