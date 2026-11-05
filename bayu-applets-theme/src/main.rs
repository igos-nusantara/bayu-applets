// SPDX-License-Identifier: GPL-3.0-only
//
// Bayu's compositor runs this with the toolkit theme's colours, and with
// XDG_CONFIG_HOME set to the applets' own configuration, before it starts the
// applets: they read the COSMIC theme there, so they look like Bayu.
//
//   bayu-applets-theme mode=dark accent=3385ff light-bg=f5f5f5 light-text=000000
//       dark-bg=1e1e1e dark-text=ffffff radii=8,10,12,14 font="Noto Sans" icons=Bayu

use std::collections::HashMap;

use cosmic_config::CosmicConfigEntry;
use cosmic_theme::palette::{Srgb, Srgba};
use cosmic_theme::{CornerRadii, Theme, ThemeBuilder, ThemeMode};

fn colour(hex: &str) -> Srgb {
    let value = u32::from_str_radix(hex.trim_start_matches('#'), 16).unwrap_or(0);
    let channel = |shift: u32| ((value >> shift) & 0xff) as f32 / 255.0;
    Srgb::new(channel(16), channel(8), channel(0))
}

fn theme(dark: bool, args: &HashMap<String, String>, radii: CornerRadii) -> Theme {
    let side = if dark { "dark" } else { "light" };
    let builder = if dark { ThemeBuilder::dark() } else { ThemeBuilder::light() };
    let mut builder = builder.corner_radii(radii);
    if let Some(accent) = args.get("accent") {
        builder = builder.accent(colour(accent));
    }
    if let Some(bg) = args.get(&format!("{side}-bg")) {
        let c = colour(bg);
        builder = builder.bg_color(Srgba::new(c.red, c.green, c.blue, 1.0));
    }
    if let Some(text) = args.get(&format!("{side}-text")) {
        builder = builder.text_tint(colour(text));
    }
    builder.build()
}

// Bayu's small, medium, big and huge radii; its buttons are rounded, not pills.
fn radii(text: Option<&String>) -> CornerRadii {
    let r: Vec<f32> = text
        .map(|t| t.split(',').filter_map(|v| v.trim().parse().ok()).collect())
        .unwrap_or_default();
    let at = |i: usize, fallback: f32| [r.get(i).copied().unwrap_or(fallback); 4];
    CornerRadii {
        radius_0: [0.0; 4],
        radius_xs: [4.0; 4],
        radius_s: at(0, 8.0),
        radius_m: at(1, 10.0),
        radius_l: at(2, 12.0),
        radius_xl: at(3, 14.0),
    }
}

// libcosmic's toolkit settings are one RON value per file, in
// $XDG_CONFIG_HOME/cosmic/com.system76.CosmicTk/v1.
fn write_tk(key: &str, ron: &str) {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config")));
    let Some(base) = base else { return };
    let dir = base.join("cosmic/com.system76.CosmicTk/v1");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join(key), ron);
}

fn main() {
    let args: HashMap<String, String> = std::env::args()
        .skip(1)
        .filter_map(|a| a.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
        .collect();
    let dark = args.get("mode").is_some_and(|m| m == "dark");
    let radii = radii(args.get("radii"));

    let mut failed = false;
    let mut write = |result: Result<(), cosmic_config::Error>, what: &str| {
        if let Err(error) = result {
            eprintln!("bayu-applets-theme: {what}: {error}");
            failed = true;
        }
    };
    match (Theme::dark_config(), Theme::light_config(), ThemeMode::config()) {
        (Ok(dark_config), Ok(light_config), Ok(mode_config)) => {
            write(theme(true, &args, radii.clone()).write_entry(&dark_config), "dark theme");
            write(theme(false, &args, radii).write_entry(&light_config), "light theme");
            let mode = ThemeMode { is_dark: dark, auto_switch: false };
            write(mode.write_entry(&mode_config), "mode");
        }
        _ => write(Err(cosmic_config::Error::NoConfigDirectory), "config"),
    }
    if let Some(icons) = args.get("icons") {
        write_tk("icon_theme", &format!("{icons:?}"));
    }
    if let Some(font) = args.get("font").filter(|f| !f.is_empty()) {
        write_tk(
            "interface_font",
            &format!("(family: {font:?}, weight: Normal, stretch: Normal, style: Normal)"),
        );
    }
    std::process::exit(if failed { 1 } else { 0 });
}
