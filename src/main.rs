use clap::Parser;
#[allow(unused_imports)]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use env_logger::Builder;
use env_logger::WriteStyle;
use log::{LevelFilter, debug};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::fs::File;
use std::io;

mod app;
mod ui;
use app::App;

#[derive(Parser)]
#[command(author, version, about, long_about = None,
    name = "SPLTUI")]

struct Args {
    #[arg(long)]
    splsv: bool,
    #[arg(long)]
    spldv: bool,
    #[arg(long)]
    hasil: bool,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    if args.verbose {
        setup_logging()?;
        debug!("Starting application in verbose mode");
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run main loop
    let mut app = App::new(args.splsv, args.spldv, args.hasil);
    run_app(&mut terminal, &mut app, args.verbose)?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    Ok(())
}

fn setup_logging() -> io::Result<()> {
    let log_file = File::create("ratatui_app.log")?;

    Builder::new()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    verbose: bool,
) -> io::Result<()> {
    if verbose {
        debug!("Entering application main loop");
    }

    loop {
        terminal.draw(|f| {
            if !verbose {
                ui::draw(f, app)
            } else {
                ui::draw_verbose(f, app);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if verbose {
                debug!("Key pressed: {:?}", key.code);
            }

            app.on_key(key.code);

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
