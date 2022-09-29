use ab_glyph::{FontRef, Font, Glyph, point, ScaleFont};

pub struct TextEngine {
    pub default_font : FontRef<'static>
}

impl TextEngine {
    pub fn new() -> TextEngine {
        let font = FontRef::try_from_slice(include_bytes!("../../assets/fonts/SecularOne-Regular.ttf")).expect("font loading failed..");
        TextEngine {
            default_font : font
        }
    }

    pub fn layout(&self, message : &str, scale : f32) -> Vec<Glyph> {
        let font = self.default_font.as_scaled(scale);

        let mut caret = point(0.0, font.ascent());
        let mut last_glyph: Option<Glyph> = None;
        let mut target = Vec::new();
        for c in message.chars() {
            if c.is_control() {
                continue;
            }
            let mut glyph = font.scaled_glyph(c);
            if let Some(previous) = last_glyph.take() {
                caret.x += font.kern(previous.id, glyph.id);
            }
            glyph.position = caret;

            last_glyph = Some(glyph.clone());
            caret.x += font.h_advance(glyph.id);

            target.push(glyph);
        }
        target
    }

    pub fn layout_data(&self, message : &str, scale : f32) -> (u32, u32, Vec<u8>) {
        let glyphs = self.layout(message, scale);

        let last_glyph = glyphs.last().unwrap();
        let width = (last_glyph.position.x + self.default_font.as_scaled(scale).h_advance(last_glyph.id)) as u32;
        let height = self.default_font.as_scaled(scale).height().ceil() as u32;

        let mut pixel_data = vec![0; (width * height * 4) as usize];
        for glyph in glyphs {
            if let Some(outline) = self.default_font.outline_glyph(glyph) {
                let bounds = outline.px_bounds();
                let left = bounds.min.x as u32;
                let top = bounds.min.y as u32;
                outline.draw(|x, y, c| {
                    let actual_x = left + x;
                    let actual_y = top + y;
                    let coverage = (c * 255.0) as u8;
                    let start_index = ((actual_y * width + actual_x) * 4) as usize;
                    pixel_data[start_index] = coverage;
                    pixel_data[start_index + 1] = coverage;
                    pixel_data[start_index + 2] = coverage;
                    pixel_data[start_index + 3] = coverage;
                });
            }
        }
        (width, height, pixel_data)
    }
}