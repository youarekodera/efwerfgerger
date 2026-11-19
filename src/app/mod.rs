use gpui::{
    App, AppContext, Application, KeyBinding, TitlebarOptions, WindowBounds, WindowOptions,
    actions, px, size,
};

use crate::ui::workspace::{OpenFolder, RefreshExplorer, ToggleBottomPanel, Workspace};

actions!(app, [Quit]);

pub fn run() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("ctrl-o", OpenFolder, None),
            KeyBinding::new("ctrl-shift-r", RefreshExplorer, None),
            KeyBinding::new("ctrl-j", ToggleBottomPanel, None),
        ]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::centered(size(px(1440.), px(900.)), cx)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Code Time".into()),
                    ..Default::default()
                }),
                window_min_size: Some(size(px(960.), px(600.))),
                app_id: Some("dev.codetime.CodeTime".into()),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let focus_handle = cx.focus_handle();
                    focus_handle.focus(window);
                    Workspace::new(focus_handle)
                })
            },
        )
        .expect("no se pudo abrir la ventana principal");

        cx.activate(true);
    });
}
