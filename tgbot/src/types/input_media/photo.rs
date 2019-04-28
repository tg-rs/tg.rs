use crate::types::ParseMode;
use serde::Serialize;

/// Photo to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaPhoto {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
}

impl InputMediaPhoto {
    /// Caption of the photo to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Set parse mode
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(
                InputMediaPhoto::default()
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
            )
            .unwrap(),
            serde_json::json!({
                "caption": "caption",
                "parse_mode": "Markdown"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaPhoto::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
