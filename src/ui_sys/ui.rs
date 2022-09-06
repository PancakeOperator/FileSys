use crate::app_state_derived_lenses::text;
use druid::kurbo::Circle;
use druid::kurbo::Size;
use druid::widget::{Align, Container, Label, Padding, Split};
use druid::widget::{Flex,  Painter, TextBox, Tabs};
use druid::{
    AppLauncher, Color, LinearGradient, LocalizedString, PlatformError, RenderContext, UnitPoint, Lens,
    Widget, WidgetExt, WindowDesc,
    Data
};

#[derive(Data, Clone, Lens)]
pub struct AppState {
   pub text: String,
}
pub fn run_app() -> impl Widget<AppState>{
    /*
    let draggable = Padding::new(
        10.0,
        Container::new(
            Split::columns(
                Align::centered(Label::new("Split A")),
                Align::centered(TextBox::multiline().lens(AppState::text)),
            )
            .split_point(0.5)
            .draggable(true)
            .solid_bar(true)
            .min_size(60.0, 60.0),
        )
        .border(Color::WHITE, 1.0),
    );
    */
    
    Padding::new(
        40.0,
        Container::new(
            Split::columns(
                Align::centered(Label::new("hmm")),
                Align::centered(TextBox::multiline().with_text_size(20.0).lens(AppState::text))
            )
            .split_point(0.5)
            .draggable(true)
            .solid_bar(true)
            .min_size(60.0, 60.0),
        )
    )
}