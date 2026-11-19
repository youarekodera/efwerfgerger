use gpui::{Hsla, rgb};

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub background: Hsla,
    pub surface: Hsla,
    pub sidebar: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub accent: Hsla,
    pub hover: Hsla,
    pub tab_active: Hsla,
    pub tab_inactive: Hsla,
    pub statusbar: Hsla,
    pub danger: Hsla,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: rgb(0x0f1117).into(),
            surface: rgb(0x171a23).into(),
            sidebar: rgb(0x12151d).into(),
            border: rgb(0x2a2f3d).into(),
            text: rgb(0xe6e9ef).into(),
            text_muted: rgb(0x9aa3b2).into(),
            accent: rgb(0x6ea8fe).into(),
            hover: rgb(0x222738).into(),
            tab_active: rgb(0x1d2230).into(),
            tab_inactive: rgb(0x151923).into(),
            statusbar: rgb(0x0b0d12).into(),
            danger: rgb(0xf07178).into(),
        }
    }
}
