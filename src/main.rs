use std::error::Error;
use std::result::Result;
use xosd_rs::{Command, HorizontalAlign, VerticalAlign, Xosd};

trait XosdDrawLines {
    fn draw_lines(
        lines: &Vec<&str>,
        horizontal_offset: i32,
        vertical_offset: i32,
        alignment: (VerticalAlign, HorizontalAlign),
    ) -> xosd_rs::Result<()>;
}

impl XosdDrawLines for Xosd {
    fn draw_lines(
        lines: &Vec<&str>,
        horizontal_offset: i32,
        vertical_offset: i32,
        alignment: (VerticalAlign, HorizontalAlign),
    ) -> xosd_rs::Result<()> {

        for line in lines.iter() {
            println!("{}", line);
        }
        Ok(())
    }
}



fn decode_alignment(number: u8) -> (VerticalAlign, HorizontalAlign) {
    let vert = number / 3;
    let horiz = number % 3;

    let vert = [
        VerticalAlign::Top,
        VerticalAlign::Center,
        VerticalAlign::Bottom,
    ][vert as usize];
    let horiz = [
        HorizontalAlign::Left,
        HorizontalAlign::Center,
        HorizontalAlign::Right,
    ][horiz as usize];

    (vert, horiz)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("res/config.toml"))?;

    let horiz_offset = settings.get_int("horizontal-offset")?;
    let vert_offset = settings.get_int("vertical-offset")?;
    let alignment = decode_alignment(settings.get_int("alignment")? as u8);

    let mut osd = Xosd::new(2)?;

    osd.draw_lines(&["blaaa", "bla"]);

    Ok(())
}
