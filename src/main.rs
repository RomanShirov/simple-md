use gpui::{
    App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x20242b))
            .text_color(rgb(0xf0f0f0))
            .text_3xl()
            .child("Hello World")
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.on_window_closed(|cx| cx.quit()).detach();

        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Simple MD".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        if let Err(error) = cx.open_window(options, |_, cx| cx.new(|_| HelloWorld)) {
            eprintln!("Failed to open Simple MD: {error}");
            cx.quit();
            return;
        }

        cx.activate(true);
    });
}
