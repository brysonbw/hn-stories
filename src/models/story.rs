use std::fmt;

use chroma_print::Color;
use clap::ValueEnum;
use serde::Deserialize;

use crate::utils::constants::{ORANGE_COLOR, Y_COMBINATOR_BASE_URL};

/// Hacker News story type (new, top, best, ask, show, jobs)
#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum HackerNewsStoryType {
    #[value(alias = "new")]
    N,

    #[value(alias = "top")]
    T,

    #[value(alias = "best")]
    B,

    #[value(alias = "ask")]
    A,

    #[value(alias = "show")]
    S,

    #[value(alias = "jobs")]
    J,
}

impl HackerNewsStoryType {
    /// Get the full name of the story kind
    pub fn full_name(&self) -> &str {
        return match self {
            HackerNewsStoryType::N => "New",
            HackerNewsStoryType::T => "Top",
            HackerNewsStoryType::B => "Best",
            HackerNewsStoryType::A => "Ask",
            HackerNewsStoryType::S => "Show",
            HackerNewsStoryType::J => "Jobs",
        };
    }

    /// Get the parameter name for the story kind
    pub fn parameter_name(&self) -> &str {
        return match self {
            HackerNewsStoryType::N => "newstories",
            HackerNewsStoryType::T => "topstories",
            HackerNewsStoryType::B => "beststories",
            HackerNewsStoryType::A => "askstories",
            HackerNewsStoryType::S => "showstories",
            HackerNewsStoryType::J => "jobstories",
        };
    }

    /// Get the maximum story limit for the story kind
    pub fn max_stories_limit(&self) -> u16 {
        return match self {
            HackerNewsStoryType::N | HackerNewsStoryType::T | HackerNewsStoryType::B => 500,
            _ => 200,
        };
    }
}

/// Actions available to the user when interacting with a story item
pub enum StoryAction {
    OpenUrl(String),
    ViewStoryItem(u64),
    ViewAuthor(String),
    Back,
}

impl fmt::Display for StoryAction {
    /// Formats the action for readable output
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::OpenUrl(url) => write!(f, "Open URL: {}", url),
            Self::ViewStoryItem(id) => write!(
                f,
                "View story on Hacker News: {}/item?id={}",
                Y_COMBINATOR_BASE_URL, id
            ),
            Self::ViewAuthor(author) => write!(
                f,
                "View author on Hacker News: {}/user?id={}",
                Y_COMBINATOR_BASE_URL, author
            ),
            Self::Back => write!(f, "<- Back"),
        };
    }
}

#[derive(Debug, Deserialize, Clone)]
/// Hacker News story item
pub struct StoryItem {
    pub id: u64,
    pub by: Option<String>,
    pub url: Option<String>,
    pub score: Option<i32>,
    pub title: Option<String>,
}

impl StoryItem {
    /// Get plural or singular `points` label for the story's score
    fn get_points_text(&self) -> String {
        let suffix: &str = if self.score.unwrap_or(0) == 1 {
            ""
        } else {
            "s"
        };
        return format!("point{suffix}");
    }

    /// Formats Hacker News story item for display within a list
    pub fn format_for_list(&self, index: usize) -> String {
        return format!(
            "{}. {} ({}) [{} {} by {}]",
            index,
            self.title.as_deref().unwrap_or("No Title"),
            self.url.as_deref().unwrap_or("No URL"),
            self.score.unwrap_or(0),
            self.get_points_text(),
            self.by.as_deref().unwrap_or("Unknown")
        );
    }

    /// Formats Hacker News story item for the detailed view of the item
    pub fn format_for_details_header(&self) -> String {
        return format!(
            "{}{} [{} {} by {}]. What would you like to do?{}",
            ORANGE_COLOR,
            self.title.as_deref().unwrap_or("No Title"),
            self.score.unwrap_or(0),
            self.get_points_text(),
            self.by.as_deref().unwrap_or("Unknown"),
            Color::Reset.value()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hacker_news_story_type_full_name() {
        assert_eq!(HackerNewsStoryType::N.full_name(), "New");
        assert_eq!(HackerNewsStoryType::T.full_name(), "Top");
        assert_eq!(HackerNewsStoryType::B.full_name(), "Best");
        assert_eq!(HackerNewsStoryType::A.full_name(), "Ask");
        assert_eq!(HackerNewsStoryType::S.full_name(), "Show");
        assert_eq!(HackerNewsStoryType::J.full_name(), "Jobs");
    }
    #[test]
    fn test_hacker_news_story_type_parameter_name() {
        assert_eq!(HackerNewsStoryType::N.parameter_name(), "newstories");
        assert_eq!(HackerNewsStoryType::T.parameter_name(), "topstories");
        assert_eq!(HackerNewsStoryType::B.parameter_name(), "beststories");
        assert_eq!(HackerNewsStoryType::A.parameter_name(), "askstories");
        assert_eq!(HackerNewsStoryType::S.parameter_name(), "showstories");
        assert_eq!(HackerNewsStoryType::J.parameter_name(), "jobstories");
    }

    #[test]
    fn test_hacker_news_story_type_max_stories_limit() {
        assert_eq!(HackerNewsStoryType::N.max_stories_limit(), 500);
        assert_eq!(HackerNewsStoryType::T.max_stories_limit(), 500);
        assert_eq!(HackerNewsStoryType::B.max_stories_limit(), 500);
        assert_eq!(HackerNewsStoryType::A.max_stories_limit(), 200);
        assert_eq!(HackerNewsStoryType::S.max_stories_limit(), 200);
        assert_eq!(HackerNewsStoryType::J.max_stories_limit(), 200);
    }

    #[test]
    fn test_story_action_display() {
        // Back
        assert_eq!(format!("{}", StoryAction::Back), "<- Back");

        // StoryItem
        assert!(format!("{}", StoryAction::ViewStoryItem(47074735)).contains("item?id=47074735"));

        // OpenUrl
        let url = "https://google.com".to_string();
        assert_eq!(
            format!("{}", StoryAction::OpenUrl(url)),
            "Open URL: https://google.com"
        );

        // ViewAuthor
        assert!(
            format!("{}", StoryAction::ViewAuthor("jake".to_string())).contains("user?id=jake")
        );
    }

    #[test]
    fn test_get_points_text() {
        let mut item: StoryItem = StoryItem {
            id: 1,
            by: None,
            url: None,
            score: Some(1),
            title: None,
        };

        // Singular
        assert_eq!(item.get_points_text(), "point");

        item.score = Some(0); // Plural (0 points)
        assert_eq!(item.get_points_text(), "points");

        item.score = Some(100); // Plural
        assert_eq!(item.get_points_text(), "points");

        item.score = None; // Default
        assert_eq!(item.get_points_text(), "points");
    }

    #[test]
    fn test_format_for_list() {
        let item: StoryItem = StoryItem {
            id: 123,
            by: Some("johndoe".to_string()),
            url: Some("https://google.com".to_string()),
            score: Some(456),
            title: Some("Title test".to_string()),
        };

        let formatted = item.format_for_list(1);

        assert!(formatted.starts_with("1. Title test"));
        assert!(formatted.contains("(https://google.com)"));
        assert!(formatted.contains("[456 points by johndoe]"));
    }

    #[test]
    fn test_format_item_with_missing_data() {
        let item: StoryItem = StoryItem {
            id: 123,
            by: None,
            url: None,
            score: None,
            title: None,
        };

        let formatted: String = item.format_for_list(1);

        assert!(formatted.contains("No Title"));
        assert!(formatted.contains("Unknown"));
        assert!(formatted.contains("0 points"));
    }
}
