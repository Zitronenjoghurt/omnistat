use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
/// https://www.who.int/news-room/questions-and-answers/item/radiation-the-ultraviolet-(uv)-index
pub struct UVIndex(f32);

impl UVIndex {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
}
