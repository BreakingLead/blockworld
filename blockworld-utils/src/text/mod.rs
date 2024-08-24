//! DO NOT CARE ABOUT THIS FILE.
//! THIS IS TOTALLY UNUSED CODE.
//! I DONT KNOW WHY I PUT IT HERE.
use crate::resource_location::ResourceLocation;

struct ClickEvent;
struct HoverEvent;

struct Style {
    pub color: i32,
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
    pub font_id: ResourceLocation,
    pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: 0,
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            font_id: todo!(),
            click_event: None,
            hover_event: None,
        }
    }
}

trait AbstractTextComponent {
    fn get_string(&self) -> String;
}
