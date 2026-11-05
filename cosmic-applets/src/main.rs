// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt().with_env_filter("warn").init();
    let _ = tracing_log::LogTracer::init();

    let Some(applet) = std::env::args().next() else {
        return Ok(());
    };

    let start = applet.rfind('/').map_or(0, |v| v + 1);
    let cmd = &applet.as_str()[start..];

    tracing::info!("Starting `{cmd}` with version {VERSION}");

    match cmd {
        "cosmic-applet-a11y" => cosmic_applet_a11y::run(),
        "cosmic-applet-workspaces" => cosmic_applet_workspaces::run(),
        "cosmic-applet-input-sources" => cosmic_applet_input_sources::run(),
        _ => Ok(()),
    }
}
