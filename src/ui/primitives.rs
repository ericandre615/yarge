use crate::ui::component::{Component, Components, ComponentProps, PropPosition, PropColor};
use crate::font;

#[derive(Debug, Copy, Clone)]
pub struct ViewProps {
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub left: f32,
    pub color: PropColor,
}

impl ComponentProps for ViewProps {
    fn width(&self) -> f32 { self.width }
    fn height(&self) -> f32 { self.height }
    fn position(&self) -> (f32, f32) { (self.left, self.top) }
    fn color(&self) -> PropColor { self.color }
}

#[derive(Debug, Clone)]
pub struct View {
    pub props: ViewProps,
    pub children: Components,
}

impl Component for View {
    fn name(&self) -> String { "ViewComp".to_string() }
    fn id(&self) -> u32 { 1 }
    fn render(&self) -> String { "RenderingView".to_string() }
    fn props(&self) -> Box<dyn ComponentProps> { Box::new(self.props) }
    fn children(&self) -> Components { self.children.clone() }
}

#[derive(Debug, Clone)]
pub struct TextProps {
    pub text: String,
    pub settings: font::TextSettings,
}

impl ComponentProps for TextProps {
    fn width(&self) -> f32 { self.settings.width }
    fn height(&self) -> f32 { 0.0 } // how to get this???
    fn position(&self) -> (f32, f32) {self.settings.pos }
    fn color(&self) -> PropColor { self.settings.color }
}

#[derive(Debug, Clone)]
pub struct Text {
    props: TextProps,
    text: font::Text,
}

impl Component for Text {
    fn name(&self) -> String { "TextComponent".to_string() }
    fn id(&self) -> u32 { 2 }
    fn render(&self) -> String { "RenderingViewText".to_string() }
    fn props(&self) -> Box<dyn ComponentProps> { Box::new(self.props.clone()) }
    fn children(&self) -> Components { Vec::new() }
}

impl Text {
    pub fn new(props: TextProps) -> Self {
        let settings = props.settings.clone();
        let text = props.text.clone();

        Self {
            props,
            text: font::Text::new(text, settings)
        }
    }
}

pub struct Button {

}

pub struct Input {

}


