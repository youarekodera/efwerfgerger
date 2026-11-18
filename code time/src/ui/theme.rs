use gpui::{Hsla, rgb};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub background: Hsla,
    pub surface: Hsla,
    pub sidebar: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub accent: Hsla,
    pub accent_text: Hsla,
    pub hover: Hsla,
    pub selection: Hsla,
    pub tab_active: Hsla,
    pub tab_inactive: Hsla,
    pub statusbar: Hsla,
    pub danger: Hsla,
}

impl Theme {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self {
                background: rgb(0x0f1117).into(),
                surface: rgb(0x171a23).into(),
                sidebar: rgb(0x12151d).into(),
                border: rgb(0x2a2f3d).into(),
                text: rgb(0xe6e9ef).into(),
                text_muted: rgb(0x9aa3b2).into(),
                accent: rgb(0x6ea8fe).into(),
                accent_text: rgb(0x0f1117).into(),
                hover: rgb(0x222738).into(),
                selection: Hsla::from(rgb(0x6ea8fe)).opacity(0.25),
                tab_active: rgb(0x1d2230).into(),
                tab_inactive: rgb(0x151923).into(),
                statusbar: rgb(0x0b0d12).into(),
                danger: rgb(0xf07178).into(),
            },
            ThemeMode::Light => Self {
                background: rgb(0xf6f7f9).into(),
                surface: rgb(0xffffff).into(),
                sidebar: rgb(0xeceff4).into(),
                border: rgb(0xd6dae2).into(),
                text: rgb(0x1f2430).into(),
                text_muted: rgb(0x667085).into(),
                accent: rgb(0x2563eb).into(),
                accent_text: rgb(0xffffff).into(),
                hover: rgb(0xe3e8ef).into(),
                selection: Hsla::from(rgb(0x2563eb)).opacity(0.18),
                tab_active: rgb(0xffffff).into(),
                tab_inactive: rgb(0xeceff4).into(),
                statusbar: rgb(0xe3e8ef).into(),
                danger: rgb(0xb42318).into(),
            },
        }
    }
}
