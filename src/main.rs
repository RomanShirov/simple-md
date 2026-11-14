mod markdown;

use gpui_kit::base::{ColorTokens, Theme, ThemeAppearance};
use gpui_kit::{App, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size};
use markdown::MarkdownPreview;

const EXAMPLE_MARKDOWN: &str = include_str!("../assets/example.md");

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kit::init(cx);
        let theme = Theme::global_mut(cx);
        theme.appearance = ThemeAppearance::Dark;
        theme.tokens.colors = ColorTokens::dark();
        cx.on_window_closed(|cx, _| cx.quit()).detach();

        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(500.0), px(0.0))),
            titlebar: Some(TitlebarOptions {
                title: Some("Simple MD".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        if let Err(error) = gpui_kit::open_window(options, cx, |_, cx| {
            cx.new(|cx| MarkdownPreview::new(EXAMPLE_MARKDOWN, cx))
        }) {
            eprintln!("Failed to open Simple MD: {error}");
            cx.quit();
            return;
        }

        cx.activate(true);
    });
}
