use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
use macroquad::color::Color;
use macroquad::text::draw_multiline_text;
use macroquad::time::get_frame_time;
use macroquad::window::{next_frame, screen_height, screen_width, Conf};

use scheme_3d::{build_macroquad_module, RuntimeConfig, ScriptHost, ScriptWatcher, PRELUDE_SCM};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser, Debug)]
#[command(
    name = "scheme_3d",
    version,
    about = "Macroquad 2D/3D graphics host & hot-reload runner for Steel Scheme"
)]
struct CliArgs {
    /// Directory containing main.scm and project .scm modules
    #[arg(default_value = ".")]
    script_dir: PathBuf,

    /// Window title
    #[arg(long, default_value = "scheme_3d")]
    title: String,

    /// Initial window width in logical pixels
    #[arg(long, default_value_t = 1280)]
    width: i32,

    /// Initial window height in logical pixels
    #[arg(long, default_value_t = 720)]
    height: i32,

    /// MSAA sample count (1, 2, 4, or 8)
    #[arg(long, default_value_t = 4)]
    msaa: i32,

    /// Disable automatic .scm file watching / hot-reload
    #[arg(long)]
    no_watch: bool,

    /// Start window in fullscreen mode
    #[arg(long)]
    fullscreen: bool,

    /// Enable High-DPI framebuffer scaling
    #[arg(long)]
    high_dpi: bool,
}

fn draw_error_overlay(err: &str) {
    let banner_h = 180.0_f32.min(screen_height());
    macroquad::shapes::draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        banner_h,
        Color::new(0.18, 0.03, 0.05, 0.88),
    );
    let snippet: String = err.lines().take(6).collect::<Vec<_>>().join("\n");
    draw_multiline_text(
        format!("[scheme_3d Error - save .scm to hot-reload]\n{snippet}"),
        14.0,
        24.0,
        18.0,
        Some(1.15),
        Color::new(1.0, 0.78, 0.78, 1.0),
    );
}

async fn run_frame_loop(config: RuntimeConfig) {
    let mut host = match ScriptHost::load(config.clone(), PRELUDE_SCM, build_macroquad_module) {
        Ok(host) => host,
        Err(err) => {
            eprintln!("[scheme_3d] Failed to load {:?}:\n{}", config.entry_path(), err);
            std::process::exit(1);
        }
    };

    let mut watcher = if config.watch_enabled {
        match ScriptWatcher::new(&config.script_dir) {
            Ok(w) => {
                eprintln!(
                    "[scheme_3d] Watching {:?} for .scm changes (hot-reload active)",
                    config.script_dir
                );
                Some(w)
            }
            Err(err) => {
                eprintln!("[scheme_3d] Warning: file watcher unavailable: {}", err);
                None
            }
        }
    } else {
        None
    };

    loop {
        if let Some(w) = watcher.as_mut() {
            let _ = host.poll_and_reload(w);
        }

        let dt = get_frame_time();
        let _ = host.update(dt);
        let _ = host.draw();

        if let Some(err) = host.last_error() {
            draw_error_overlay(err);
        }

        next_frame().await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args = CliArgs::parse();
    let config = RuntimeConfig::new(args.script_dir).with_watch(!args.no_watch);
    let entry_path = config.entry_path();
    if !entry_path.is_file() {
        eprintln!(
            "[scheme_3d] Entry file not found: {:?}\nUsage: scheme_3d <script_dir> (containing main.scm)",
            entry_path
        );
        std::process::exit(1);
    }

    let conf = Conf {
        window_title: args.title,
        window_width: args.width.max(100),
        window_height: args.height.max(100),
        sample_count: args.msaa.max(1),
        fullscreen: args.fullscreen,
        high_dpi: args.high_dpi,
        window_resizable: true,
        ..Default::default()
    };

    macroquad::Window::from_config(conf, run_frame_loop(config));
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let conf = Conf {
        window_title: "scheme_3d".to_string(),
        window_width: 1280,
        window_height: 720,
        sample_count: 4,
        window_resizable: true,
        ..Default::default()
    };
    let config = RuntimeConfig::new(PathBuf::from(".")).with_watch(false);
    macroquad::Window::from_config(conf, run_frame_loop(config));
}
