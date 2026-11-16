use gpui_kit::base::{TextView, TextViewState, TextViewStyle, Theme};
use gpui_kit::{
    AppContext, Context, Entity, HighlightStyle, IntoElement, Render, StyleRefinement, Window, div,
    prelude::*, px, rgb,
};

pub struct MarkdownPreview {
    document: Entity<TextViewState>,
    style: TextViewStyle,
}

impl MarkdownPreview {
    pub fn new(source: &str, cx: &mut Context<Self>) -> Self {
        let document = cx.new(|cx| TextViewState::markdown(source, cx));
        let style = TextViewStyle::from_theme(&Theme::global(cx))
            .with_foreground(rgb(0xf0f0f0).into())
            .with_muted_foreground(rgb(0xa9b4c4).into())
            .with_link(rgb(0x82aaff).into())
            .with_code_background(rgb(0x282f3a).into())
            .with_border(rgb(0x465063).into())
            .with_inline_code(HighlightStyle {
                color: Some(rgb(0xffcb8b).into()),
                background_color: Some(rgb(0x343b48).into()),
                ..Default::default()
            })
            .with_inline_code_style(
                StyleRefinement::default()
                    .px(px(4.0))
                    .py(px(1.0))
                    .rounded_sm(),
            )
            .with_table(StyleRefinement::default().bg(rgb(0x282f3a)))
            .with_table_cell(StyleRefinement::default().whitespace_normal());

        Self { document, style }
    }

    pub fn set_source(&mut self, source: &str, cx: &mut Context<Self>) {
        self.document
            .update(cx, |document, cx| document.set_text(source, cx));
    }
}

impl Render for MarkdownPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .p_6()
            .overflow_hidden()
            .bg(rgb(0x20242b))
            .text_color(rgb(0xf0f0f0))
            .text_size(px(18.0))
            .line_height(px(29.0))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .min_h_0()
                    .min_w_0()
                    .overflow_hidden()
                    .child(
                        TextView::new(&self.document)
                            .size_full()
                            .scrollable(true)
                            .style(self.style.clone()),
                    ),
            )
    }
}
