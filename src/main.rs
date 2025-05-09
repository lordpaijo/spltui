use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::Rect,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Color, Stylize, Modifier},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal, Frame,
};
use matematika_rs::sistem::aljabar::*;

enum AppState {
    Menu,
    InputSPLDV([String; 6], usize),
    InputSPLSV([String; 2], usize),
    Result(String),
    Exit,
}

struct App {
    state: AppState,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::Menu,
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        match &mut self.state {
            AppState::Menu => match key {
                KeyCode::Char('1') => self.state = AppState::InputSPLSV(std::array::from_fn(|_| "".to_string()), 0),
                KeyCode::Char('2') => self.state = AppState::InputSPLDV(std::array::from_fn(|_| "".to_string()), 0),
                KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,
                _ => {}
            },
            AppState::InputSPLDV(inputs, selected) => match key {
                KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,

                KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
                    inputs[*selected].push(c);
                }
                KeyCode::Backspace => {
                    inputs[*selected].pop();
                }
                KeyCode::Left => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                KeyCode::Right => {
                    if *selected < inputs.len() - 1 {
                        *selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if let (Ok(a1), Ok(b1), Ok(c1), Ok(a2), Ok(b2), Ok(c2)) = (
                        inputs[0].parse(),
                        inputs[1].parse(),
                        inputs[2].parse(),
                        inputs[3].parse(),
                        inputs[4].parse(),
                        inputs[5].parse(),
                    ) {
                        let (result, steps) = SistemPersamaan::spldv_proses(a1, b1, c1, a2, b2, c2);
                        let result_str = result
                            .map(|(x, y)| format!("{steps}\n\nHasil:\n  x = {:.2}, y = {:.2}", x, y))
                            .unwrap_or_else(|| format!("{steps}\n\nTidak ada solusi"));
                        self.state = AppState::Result(result_str);
                    } else {
                        self.state = AppState::Result("Input tidak valid".to_string());
                    }
                }
                KeyCode::Esc => self.state = AppState::Menu,
                _ => {}
            },
            AppState::InputSPLSV(inputs, selected) => match key {
                KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,

                KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
                    inputs[*selected].push(c);
                }
                KeyCode::Backspace => {
                    inputs[*selected].pop();
                }
                KeyCode::Left => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                KeyCode::Right => {
                    if *selected < inputs.len() - 1 {
                        *selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if let (Ok(a), Ok(b)) = (inputs[0].parse(), inputs[1].parse()) {
                        let (result, steps) = SistemPersamaan::splsv_proses(a, b);
                        let result_str = match result {
                            Some(x) => format!("{steps}\n\nHasil:\n  x = {:.2}", x),
                            None => format!("{steps}\n\nTidak ada solusi"),
                        };
                        self.state = AppState::Result(result_str);
                    } else {
                        self.state = AppState::Result("Input tidak valid".to_string());
                    }
                }
                KeyCode::Esc => self.state = AppState::Menu,
                _ => {}
            },
            AppState::Result(_) => match key {
                KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,
                KeyCode::Esc => self.state = AppState::Menu,
                _ => {}
            },
            AppState::Exit => {}
        }
    }

    fn ui(&self, f: &mut ratatui::Frame<'_>) {
        let owner = Line::styled(format!(" {}'s {} v{} ", "LordPaijo", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            Style::default().fg(Color::Cyan).bold());

        let ascii_lines = vec![
            Line::from(vec![
                Span::styled("░██████╗██████╗░██╗░░░░░", Style::default().fg(Color::Yellow)),
                Span::styled("████████╗██╗░░░██╗██╗", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("██╔════╝██╔══██╗██║░░░░░", Style::default().fg(Color::Yellow)),
                Span::styled("╚══██╔══╝██║░░░██║██║", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("╚█████╗░██████╔╝██║░░░░░", Style::default().fg(Color::Yellow)),
                Span::styled("░░░██║░░░██║░░░██║██║", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("░╚═══██╗██╔═══╝░██║░░░░░", Style::default().fg(Color::Yellow)),
                Span::styled("░░░██║░░░██║░░░██║██║", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("██████╔╝██║░░░░░███████╗", Style::default().fg(Color::Yellow)),
                Span::styled("░░░██║░░░╚██████╔╝██║", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("╚═════╝░╚═╝░░░░░╚══════╝", Style::default().fg(Color::Yellow)),
                Span::styled("░░░╚═╝░░░░╚═════╝░╚═╝", Style::default().fg(Color::Green)),
            ]),
        ];

        fn render_menu_ui(f: &mut Frame, area: ratatui::prelude::Rect, ascii_lines: &Vec<Line<'_>>, owner: &Line<'_>) {
            // Layout utama: Header dan Konten
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(8), // Header
                    Constraint::Min(0),    // Konten
                ])
                .split(area);

            // Header Title
            let title = Paragraph::new(Text::from(ascii_lines.to_vec()))
                .style(Style::default())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default())
                        .fg(Color::Rgb(254, 128, 25))
                        .title(" Header ").title_bottom(owner.clone().centered()),
                )
                .wrap(Wrap { trim: false });

            let instructions = Line::from(vec![
                Span::styled(" [Q] ", Style::default().fg(Color::Red).bold()),
                Span::styled("Quit ", Style::default().fg(Color::Red).bold()),
            ]);

            let block = Block::bordered()
                .title_bottom(instructions.centered())
                .border_set(border::THICK)
                .border_style(Style::default().fg(Color::Cyan));

            // Menu utama
            let menu = Paragraph::new(Line::from(vec![
                Span::raw("\n"),
                Span::styled("[1] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("SPLSV\n", Style::default().bold()),
                Span::styled("        [2] ", Style::default().fg(Color::Green).bold()),
                Span::styled("SPLDV\n", Style::default().bold()),
            ]))
            .alignment(Alignment::Center)
            .block(block.borders(Borders::ALL).title(" Menu "));

            f.render_widget(title, chunks[0]);
            f.render_widget(menu, chunks[1]);
        }

        fn render_input_spldv_ui(f: &mut Frame, area: Rect, inputs: &[String; 6], selected: usize,
            ascii_lines: &Vec<Line<'_>>, owner: &Line<'_>) {
            let outer_block = Block::default()
                .title("Form SPLDV")
                .borders(Borders::ALL);
            f.render_widget(outer_block, area);

            let outer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(8), // Header di luar
                    Constraint::Min(0), // Sisanya untuk kontainer
                ])
                .split(area);

            // Header
            let header = Paragraph::new(Text::from(ascii_lines.to_vec()))
                .style(Style::default())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default())
                        .fg(Color::Rgb(254, 128, 25))
                        .title(" Header ").title_bottom(owner.clone().centered()),
                )
                .wrap(Wrap { trim: false });
            f.render_widget(header, outer_chunks[0]);

            // Buat blok kontainer (dengan border)
            let instructions = Line::from(vec![
                Span::styled(" [Esc] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Menu ", Style::default().fg(Color::White).bold()),
                Span::styled("[←/→] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Left/Right ", Style::default().fg(Color::White).bold()),
                Span::styled("[Enter] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Submit ", Style::default().fg(Color::White).bold()),
                Span::styled("[Q] ", Style::default().fg(Color::Red).bold()),
                Span::styled("Quit ", Style::default().fg(Color::Red).bold()),
            ]);

            let container_block = Block::default()
                .title_bottom(instructions.centered())
                .title(" Input SPLDV ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));
            let inner_area = container_block.inner(outer_chunks[1]); // Ambil area dalamnya
            f.render_widget(container_block, outer_chunks[1]);

            // Di dalam inner_area, buat layout lagi untuk title dan inputs
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4), // Title row
                    Constraint::Length(6), // Row 1 input
                    Constraint::Length(6), // Row 2 input
                ])
                .split(inner_area);

            let title = Paragraph::new(Text::from(vec![
                Line::from(Span::styled("Persamaan Linear Dua Variabel (SPLDV)", Style::default().add_modifier(Modifier::BOLD))),
                Line::from(Span::raw("a1x + b1y = c1")),
                Line::from(Span::raw("a2x + b2y = c2")),
            ])).alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            let row1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(33); 3])
                .split(chunks[1]);

            let row2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(33); 3])
                .split(chunks[2]);

            let labels = vec!["a1", "b1", "c1", "a2", "b2", "c2"];
            for (j, chunk) in [row1, row2].iter().enumerate() {
                for (k, area) in chunk.iter().enumerate() {
                    let idx = j * 3 + k;
                    let input = Paragraph::new(Line::from(vec![
                        Span::raw(format!("{}: ", labels[idx])),
                        Span::raw(&inputs[idx]),
                    ]))
                    .block(Block::default().borders(Borders::ALL)
                    .border_style(if idx == selected {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::Blue)
                    }))
                    .alignment(Alignment::Left)
                    .style(if idx == selected {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::White)
                    });
                    f.render_widget(input, *area);
                }
            }
        }

        fn render_input_splsv_ui(f: &mut Frame, area: Rect, inputs: &[String; 2], selected: usize,
            ascii_lines: &Vec<Line<'_>>, owner: &Line<'_>) {
            let outer_block = Block::default()
                .title("Form SPLDV")
                .borders(Borders::ALL);
            f.render_widget(outer_block, area);

            let outer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(8), // Header di luar
                    Constraint::Min(0),    // Sisanya untuk kontainer
                ])
                .split(area);

            // Header
            let header = Paragraph::new(Text::from(ascii_lines.to_vec()))
                .style(Style::default())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default())
                        .fg(Color::Rgb(254, 128, 25))
                        .title(" Header ").title_bottom(owner.clone().centered()),
                )
                .wrap(Wrap { trim: false });
            f.render_widget(header, outer_chunks[0]);

            // Buat blok kontainer (dengan border)
            let instructions = Line::from(vec![
                Span::styled(" [Esc] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Menu ", Style::default().fg(Color::White).bold()),
                Span::styled("[←/→] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Left/Right ", Style::default().fg(Color::White).bold()),
                Span::styled("[Enter] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Submit ", Style::default().fg(Color::White).bold()),
                Span::styled("[Q] ", Style::default().fg(Color::Red).bold()),
                Span::styled("Quit ", Style::default().fg(Color::Red).bold()),
            ]);

            let container_block = Block::default()
                .title_bottom(instructions.centered())
                .title(" Input SPLSV ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan));
            let inner_area = container_block.inner(outer_chunks[1]); // Ambil area dalamnya
            f.render_widget(container_block, outer_chunks[1]);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4), // Title row
                    Constraint::Length(6), // Row 1 input
                    Constraint::Length(6), // Row 2 input
                ])
                .split(inner_area);

            let title = Paragraph::new(Text::from(vec![
                Line::from(Span::styled("Persamaan Linear Satu Variabel (SPLSV)", Style::default().add_modifier(Modifier::BOLD))),
                Line::from(Span::raw("ax + b = 0")),
            ])).alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            let input_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50); 2])
                .split(chunks[1]);

            let labels = vec!["a", "b"];
            for (i, area) in input_row.iter().enumerate() {
                let input = Paragraph::new(Line::from(vec![
                    Span::styled(format!("{}: ", labels[i]), Style::default().bold()),
                    Span::styled(&inputs[i], Style::default().bold()),
                ]))
                .block(Block::default().borders(Borders::ALL)
                .border_style(if i == selected {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Blue).bold()
                }))
                .alignment(Alignment::Left)
                .style(if i == selected {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::White).bold()
                });
                f.render_widget(input, *area);
            }
        }

        fn render_result_ui(f: &mut Frame, area: Rect, result_text: &str, ascii_lines: &Vec<Line<'_>>, owner: &Line<'_>) {
            let outer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(8), // Header
                    Constraint::Min(0),    // Kontainer isi hasil
                ])
                .split(area);

            let header = Paragraph::new(Text::from(ascii_lines.to_vec()))
                .style(Style::default())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default())
                        .fg(Color::Rgb(254, 128, 25))
                        .title(" Header ").title_bottom(owner.clone().centered()),
                )
                .wrap(Wrap { trim: false });
            f.render_widget(header, outer_chunks[0]);

            let instructions = Line::from(vec![
                Span::styled(" [Esc] ", Style::default().fg(Color::Blue).bold()),
                Span::styled("Menu ", Style::default().fg(Color::White).bold()),
                Span::styled("[Q] ", Style::default().fg(Color::Red).bold()),
                Span::styled("Quit ", Style::default().fg(Color::Red).bold()),
            ]);

            let container_block = Block::default()
                .title(" Hasil Perhitungan ")
                .title_bottom(instructions.centered())
                .borders(Borders::ALL)
                .border_set(border::THICK)
                .border_style(Style::default().fg(Color::Cyan))
                .style(Style::default().fg(Color::White));
            let inner_area = container_block.inner(outer_chunks[1]);
            f.render_widget(container_block, outer_chunks[1]);

            let mut lines: Vec<Line> = vec![
                Line::from(vec![Span::styled("Hasil:", Style::default().fg(Color::Green).bold())]),
            ];
            lines.extend(result_text.lines().map(|line| Line::raw(line.to_string())));

            let result = Paragraph::new(Text::from(lines))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            f.render_widget(result, inner_area);
        }

        match &self.state {
            AppState::Menu => {
                render_menu_ui(f, f.area(), &ascii_lines, &owner);
            }
            AppState::InputSPLDV(inputs, selected) => {
                render_input_spldv_ui(f, f.area(), inputs, *selected, &ascii_lines, &owner);
            }
            AppState::InputSPLSV(inputs, selected) => {
                render_input_splsv_ui(f, f.area(), inputs, *selected, &ascii_lines, &owner);
            }
            AppState::Result(result) => {
                render_result_ui(f, f.area(), result, &ascii_lines, &owner);
            }
            AppState::Exit => {}
        }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| app.ui(f))?;
        if let Event::Key(key) = event::read()? {
            app.on_key(key.code);
            if matches!(app.state, AppState::Exit) {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
