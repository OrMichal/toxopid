use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    geometry::Point,
    image::{Image, ImageDrawable, ImageRaw},
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    text::Text,
};
use heapless::format;
use ssd1306::{
    Ssd1306, mode::BufferedGraphicsMode, prelude::WriteOnlyDataCommand, size::DisplaySize128x64,
};

use crate::robot::expressions::Expression;

pub struct Tox<DI> {
    expression: Expression,
    display: Ssd1306<DI, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
}

impl<DI> Tox<DI>
where
    DI: WriteOnlyDataCommand,
{
    pub fn new(
        display: Ssd1306<DI, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
    ) -> Self {
        Tox {
            expression: Expression::Idle,
            display,
        }
    }

    pub fn set_expression(&mut self, expression: Expression) {
        self.expression = expression;
    }

    pub fn get_expression(&self) -> &Expression {
        &self.expression
    }

    pub fn update_display(&mut self) {
        self.display.clear(BinaryColor::Off).unwrap();

        let charge = 90_u8;

        Text::new(
            &format!("{}%", charge).into(),
            Point::new(0, 0),
            MonoTextStyleBuilder::new()
                .font(&FONT_6X10)
                .text_color(BinaryColor::On)
                .build(),
        )
        .draw(&mut self.display)
        .unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let expression_array = self.expression.to_byte_array();

        let raw_image = ImageRaw::<BinaryColor>::new(expression_array, 128);
        let image = Image::new(&raw_image, Point::new(0, 0));
        image.draw(&mut self.display).unwrap();

        self.display.flush().unwrap();
    }
}
