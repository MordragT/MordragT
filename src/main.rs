use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::env;

pub mod convert;
pub mod logo;
pub mod profile;

use profile::{Options, Profile};

fn main() -> Result<()> {
    let token = env::var("GITHUB_TOKEN").expect("ACCESS_TOKEN environment variable not set");

    let birth = NaiveDateTime::parse_from_str("21.04.2000 22:00:00", "%d.%m.%Y %H:%M:%S")?;
    let birth = DateTime::<Utc>::from_naive_utc_and_offset(birth, Utc);

    let options = Options {
        token,
        username: "mordragt".to_owned(),
        birth: Some(birth),
        linkedin: Some("thomas-wehmoeller".to_owned()),
        os: Some("NixOS 25.11 (Xantusia) x86_64".to_owned()),
        ..Default::default()
    };

    let profile = Profile::fetch(options)?;

    let mut contents = String::new();
    profile.render(&mut contents)?;
    convert::to_svg(&contents, convert::Options::DEFAULT)?;

    print!("{contents}");

    Ok(())
}
