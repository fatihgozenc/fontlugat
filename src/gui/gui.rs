use druid::widget::{Align, Flex, Label, Padding, Stepper};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use super::component::font_card;

pub fn build() -> impl Widget<()> {
    return Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(font_card("top left"), 1.0)
                    .with_flex_child(Align::centered(font_card("bottom left")), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(font_card("top right"), 1.0)
                    .with_flex_child(Align::centered(font_card("bottom right")), 1.0),
                1.0,
            ),
    );
}
