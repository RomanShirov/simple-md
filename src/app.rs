#[cfg(not(target_os = "macos"))]
use gpui_kit::component::menu::AppMenuBar;
use gpui_kit::component::{
    Sizable,
    button::{Button, ButtonVariants},
    input::{Editor, EditorState},
};
use gpui_kit::{
    AppContext, ClickEvent, Context, Entity, IntoElement, Render, Window, div, prelude::*, px, rgb,
};

use crate::markdown::MarkdownPreview;

pub struct AppView {
    preview: Entity<MarkdownPreview>,
    editor: Entity<EditorState>,
    editing: bool,
    #[cfg(not(target_os = "macos"))]
    menu: Entity<AppMenuBar>,
}

impl AppView {
    pub fn new(source: &str, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            preview: cx.new(|cx| MarkdownPreview::new(source, cx)),
            editor: cx.new(|cx| EditorState::new(window, cx).default_value(source)),
            editing: false,
            #[cfg(not(target_os = "macos"))]
            menu: AppMenuBar::new(cx),
        }
    }

    fn toggle_editing(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.editing {
            let source = self.editor.read(cx).value();
            self.preview.update(cx, |preview, cx| {
                preview.set_source(source.as_ref(), cx);
            });
            self.editing = false;
            cx.notify();
            return;
        }

        self.editing = true;
        self.editor
            .update(cx, |editor, cx| editor.focus(window, cx));
        cx.notify();
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toolbar = div()
            .w_full()
            .h(px(34.0))
            .flex()
            .items_center()
            .gap_2()
            .flex_shrink_0()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(rgb(0x465063))
            .bg(rgb(0x282f3a))
            .text_size(px(14.0))
            .line_height(px(20.0));

        #[cfg(not(target_os = "macos"))]
        let toolbar = toolbar.child(div().flex_1().min_w_0().child(self.menu.clone()));

        #[cfg(target_os = "macos")]
        let toolbar = toolbar.child(div().flex_1());

        let toolbar = toolbar.child(
            Button::new("toggle-editing")
                .small()
                .ghost()
                .label(if self.editing { "Preview" } else { "Edit" })
                .on_click(cx.listener(Self::toggle_editing)),
        );

        let content = if self.editing {
            div()
                .size_full()
                .p_3()
                .child(
                    Editor::new(&self.editor)
                        .size_full()
                        .appearance(false)
                        .bordered(false),
                )
                .into_any_element()
        } else {
            self.preview.clone().into_any_element()
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x20242b))
            .child(toolbar)
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .min_h_0()
                    .min_w_0()
                    .overflow_hidden()
                    .child(content),
            )
    }
}
