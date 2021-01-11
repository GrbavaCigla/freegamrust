use std::error::Error;

pub fn get_settings() -> Result<config::Config, Box<dyn Error>> {
    let mut settings = config::Config::default();

    let mut settings_dir = std::env::var("HOME")?;
    settings_dir.push_str("/.config/freegamrust/config.toml");

    settings.merge(config::File::with_name(&settings_dir))?;

    Ok(settings)
}

pub fn get_settings_values(settings: &config::Config) -> Result<(i16, i16, u64), Box<dyn Error>> {
    let horiz_offset = settings.get_int("horizontal-offset")? as i16;
    let vert_offset = settings.get_int("vertical-offset")? as i16;
    let refresh_min = settings.get_int("refresh-min")? as u64;

    Ok((horiz_offset,vert_offset,refresh_min))
}