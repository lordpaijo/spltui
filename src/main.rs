use clap::Parser;
use crossterm::{
    event::{self, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log::{LevelFilter, debug, info};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

mod app;
mod ui;
mod themes {
    pub mod gruvbox_dark;
    pub mod gruvbox_light;
}
use app::App;

#[derive(Parser)]
#[command(author, version, about, long_about = None,
    name = "SPLTUI")]

struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    log: Option<PathBuf>,
    #[arg(short, long, default_value = "dark")]
    theme: String,
    #[arg(long)]
    splsv: bool,
    #[arg(long)]
    spldv: bool,
    #[arg(long)]
    hasil: bool,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let log_path = if args.verbose {
        Some(setup_logging(&args)?)
    } else {
        None
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run main loop
    let mut app = App::new(args.splsv, args.spldv, args.hasil);
    run_app(&mut terminal, &mut app, args.verbose, args.theme.clone())?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Some(path) = log_path {
        print_logs(&path)?;
    }

    Ok(())
}

fn setup_logging(args: &Args) -> io::Result<PathBuf> {
    let log_path = match &args.log {
        Some(path) => path.clone(),
        None => {
            // Use system temp directory as default location
            let mut temp_path = std::env::temp_dir();
            temp_path.push("ratatui_app.log");
            temp_path
        }
    };

    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let log_file = fs::File::create(&log_path)?;
    println!(
        "Verbose mode enabled. Logs will be written to: {}",
        log_path.display()
    );

    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_timestamp_millis()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    info!("Logging system initialized");

    Ok(log_path)
}

fn print_logs(log_path: &PathBuf) -> io::Result<()> {
    println!("\n===== Application Logs =====");

    let mut file = File::open(log_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);
    println!("===== End of Logs =====");

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    verbose: bool,
    theme: String,
) -> io::Result<()> {
    if verbose {
        debug!("Entering application main loop");
    }

    loop {
        terminal.draw(|f| {
            if !verbose {
                ui::draw(f, app, theme.clone())
            } else {
                ui::draw_verbose(f, app, theme.clone());
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if verbose {
                debug!("Key pressed: {:?}", key.code);
            }

            app.on_key(key);

            if app.should_exit() {
                if verbose {
                    debug!("Exit signal received");
                }
                break;
            }
        }
    }

    if verbose {
        debug!("Exiting application");
    }

    Ok(())
}
