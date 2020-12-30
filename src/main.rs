use serde_json::Value;
use std::error::Error;
use std::result::Result;
use xosd_rs::{Command, HorizontalAlign, VerticalAlign, Xosd};
use std::thread::sleep;
use std::time::Duration;

trait XosdDrawLines {
    fn draw_lines(
        &mut self,
        lines: &Vec<&str>,
        horizontal_offset: i32,
        vertical_offset: i32,
        alignment: (VerticalAlign, HorizontalAlign),
    ) -> xosd_rs::Result<()>;
}

impl XosdDrawLines for Xosd {
    fn draw_lines(
        &mut self,
        lines: &Vec<&str>,
        horizontal_offset: i32,
        vertical_offset: i32,
        alignment: (VerticalAlign, HorizontalAlign),
    ) -> xosd_rs::Result<()> {
        self.set_horizontal_offset(horizontal_offset)?;
        self.set_vertical_offset(vertical_offset)?;
        self.set_vertical_align(alignment.0)?;
        self.set_horizontal_align(alignment.1)?;

        for (i, line) in lines.iter().enumerate() {
            self.display(i as i32, Command::string(line)?)?;
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

fn fetch_games() -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::blocking::get("https://reddit.com/r/FreeGameFindings.json")?.text()?;
    let json_val: Value = serde_json::from_str(&response)?;

    let mut res = vec![];

    for i in 0..10 {
        let cur_str = match &json_val["data"]["children"][i]["data"]["title"] {
            Value::String(s) => s,
            _ => continue,
        };

        res.push(cur_str.to_owned());
    }

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut settings = config::Config::default();

    let mut settings_dir = std::env::var("HOME").unwrap();
    settings_dir.push_str("/.config/freegamrust/config.toml");

    settings.merge(config::File::with_name(&settings_dir))?;

    let horiz_offset = settings.get_int("horizontal-offset")? as i32;
    let vert_offset = settings.get_int("vertical-offset")? as i32;
    let alignment = decode_alignment(settings.get_int("alignment")? as u8);
    let refresh_min = settings.get_int("refresh-min")? as u64;

    loop {
        let v = fetch_games()?;
        let v: Vec<&str> = v.iter().map(|s| &**s).collect();

        let mut osd = Xosd::new(v.len() as i32)?;

        osd.draw_lines(&v, horiz_offset, vert_offset, alignment)?;

        if osd.onscreen()? {
            osd.wait_until_no_display()?;
        }

        sleep(Duration::from_secs(refresh_min * 60));
    }
}
