use serde_json::Value;
use std::error::Error;
use std::result::Result;
use std::thread::sleep;
use std::time::Duration;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use std::time::{SystemTime, UNIX_EPOCH};

mod config;
mod utils;
use utils::*;

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
    // Config
    let settings = config::get_settings()?;
    let (horiz_offset,vert_offset,refresh_min) = config::get_settings_values(&settings)?;
    let refresh_min = Duration::from_secs(refresh_min * 60);

    // Xorg stuff
    let (conn, screen_num) = x11rb::connect(None)?;
    let (depth, visualid) = choose_visual(&conn, screen_num)?;
    let atoms = AtomCollection::new(&conn)?.reply()?;
    let screen = &conn.setup().roots[screen_num];

    let win_id = utils::create_window(&conn, screen, &atoms, (horiz_offset, vert_offset), (400, 170), depth, visualid)?;

    let transparency = composite_manager_running(&conn, screen_num)?;
    if !transparency {
        eprintln!("Compositor is required to use transparency!");
    }

    conn.map_window(win_id)?;

    conn.flush()?;

    let mut start_time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let mut v = fetch_games()?;

    loop {
        let event = conn.wait_for_event()?;

        println!("Event: {:?}", event);

        if SystemTime::now().duration_since(UNIX_EPOCH)? - start_time >= refresh_min {
            start_time = SystemTime::now().duration_since(UNIX_EPOCH)?;
            v = fetch_games()?;
        }

        for (i, item) in v.iter().enumerate() {
            text_draw(&conn, screen, win_id, 10, 10 + 14 * (i + 1) as i16, item)?;
            println!("{}", 10 + 14 * (i + 1) as i16);
        }

        conn.flush()?;
        // sleep(Duration::from_secs(refresh_min * 60));
    }
}
