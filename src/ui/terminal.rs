use async_trait::async_trait;
use chroma_print::print_info;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    models::story::{HackerNewsStoryType, StoryAction, StoryItem},
    types::HnResult,
    ui::UserInterface,
    utils::constants::Y_COMBINATOR_BASE_URL,
};

pub struct TerminalUserInterface;

#[async_trait]
impl UserInterface for TerminalUserInterface {
    async fn render_stories_list(
        &self,
        stories: Vec<StoryItem>,
        story_type: &HackerNewsStoryType,
    ) -> HnResult<()> {
        // Format story items for list
        let formatted_story_items: Vec<String> = stories
            .iter()
            .enumerate()
            .map(|(index, story)| story.format_for_list(index + 1))
            .collect();

        // Story selection list loop
        loop {
            print_info!("Press Ctrl+C or Esc to exit.");
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "Select a '{}' story item to view more",
                    story_type.full_name()
                ))
                .items(&formatted_story_items)
                .default(0)
                .interact_opt()?;

            let index: usize = match selection {
                Some(i) => i,
                None => break, // User pressed Ctrl+C or Esc...exit
            };

            let selected_story: &StoryItem = &stories[index];

            // Selected story/item action loop
            loop {
                let mut actions: Vec<StoryAction> = Vec::new();

                if let Some(url) = &selected_story.url {
                    actions.push(StoryAction::OpenUrl(url.clone()));
                }

                actions.push(StoryAction::ViewStoryItem(selected_story.id));

                if let Some(by) = &selected_story.by {
                    actions.push(StoryAction::ViewAuthor(by.clone()));
                }

                actions.push(StoryAction::Back);

                // Show list of actions for selected story/item
                let action_index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(selected_story.format_for_details_header())
                    .items(&actions)
                    .default(0)
                    .interact()?;

                // Handle action
                match &actions[action_index] {
                    StoryAction::OpenUrl(url) => {
                        let _ = webbrowser::open(url);
                    }
                    StoryAction::ViewAuthor(user) => {
                        let _ = webbrowser::open(&format!(
                            "{}/user?id={}",
                            Y_COMBINATOR_BASE_URL, user
                        ));
                    }
                    StoryAction::ViewStoryItem(id) => {
                        let _ =
                            webbrowser::open(&format!("{}/item?id={}", Y_COMBINATOR_BASE_URL, id));
                    }
                    StoryAction::Back => break,
                }
            }
        }

        return Ok(());
    }
}
