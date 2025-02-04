use super::{SectionItem, Status};

use colored::Colorize;

pub fn items() -> Vec<SectionItem> {
  vec![SectionItem::new(
    || {
      let teams = tauri_mobile::apple::teams::find_development_teams().unwrap_or_default();
      Some((
        if teams.is_empty() {
          "None".red().to_string()
        } else {
          teams
            .iter()
            .map(|t| format!("{} (ID: {})", t.name, t.id))
            .collect::<Vec<String>>()
            .join(", ")
        },
        Status::Neutral,
      ))
    },
    || None,
    false,
  )]
}
