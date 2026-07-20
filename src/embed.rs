use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum EmbedError {
    #[error("Value `{0}` cannot be converted to hex color")]
    InvalidColor(u32),
}

#[derive(Serialize)]
pub struct Body {
    fields: Vec<Field>,
    title: String,
    color: u32, // hex color code converted to decimal
}

impl Body {
    pub fn new(title: String, color: u32) -> Result<Self, EmbedError> {
        if color > 0xffffff {
            return Err(EmbedError::InvalidColor(color));
        }

        Ok(Body {
            fields: Vec::new(),
            title,
            color,
        })
    }

    pub fn add_field(&mut self, name: String, value: String, inline: bool) {
        self.fields.push(Field {
            name,
            value,
            inline,
        });
    }
}

#[derive(Serialize)]
pub struct Field {
    name: String,
    value: String,
    inline: bool,
}
