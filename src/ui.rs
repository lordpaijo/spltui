use ratatui::{
    prelude::Rect,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Color, Stylize, Modifier},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState};

/// Main draw function that delegates to specific UI renderers based on app state
pub fn draw(f: &mut Frame, app: &App) {
    let ascii_lines = create_ascii_header();
    let owner = create_owner_line();

    match &app.state {
        AppState::Menu => render_menu_ui(f, f.area(), &ascii_lines, &owner),
        AppState::InputSPLDV(inputs, selected) => {
            render_input_spldv_ui(f, f.area(), inputs, *selected, &ascii_lines, &owner)
        }
        AppState::InputSPLSV(inputs, selected) => {
            render_input_splsv_ui(f, f.area(), inputs, *selected, &ascii_lines, &owner)
        }
        AppState::Result(result) => render_result_ui(f, f.area(), result, &ascii_lines, &owner),
        AppState::Exit => {}
    }
}

/// Creates the ASCII art header lines
fn create_ascii_header() -> Vec<Line<'static>> {
    vec![
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
    ]
}

/// Creates the owner line for the footer
fn create_owner_line() -> Line<'static> {
    Line::styled(
        format!(
            " {} (2025) {} v{} ",
            "LordPaijo",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
        Style::default().fg(Color::Cyan).bold(),
    )
}

/// Renders the header block with ASCII art
fn render_header(f: &mut Frame, area: Rect, ascii_lines: &Vec<Line<'_>>, owner: &Line<'_>) {
    let header = Paragraph::new(Text::from(ascii_lines.to_vec()))
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default())
                .fg(Color::Rgb(254, 128, 25))
                .title(" Header ").bold()
                .title_bottom(owner.clone().centered()),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(header, area);
}

/// Renders the main menu UI
fn render_menu_ui(f: &mut Frame, area: Rect, ascii_lines: &Vec<Line<'_>>, owner: &Line<'_>) {
    // Layout utama: Header dan Konten
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(8), // Header
            Constraint::Min(0), // Konten
        ])
        .split(area);

    // Header Title
    render_header(f, chunks[0], ascii_lines, owner);

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
    .block(block.borders(Borders::ALL).title(" Menu ").bold());

    f.render_widget(menu, chunks[1]);
}

/// Renders the SPLDV input form UI
fn render_input_spldv_ui(
    f: &mut Frame,
    area: Rect,
    inputs: &[String; 6],
    selected: usize,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
) {
    let outer_block = Block::default().title("Form SPLDV").borders(Borders::ALL);
    f.render_widget(outer_block, area);

    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(8), // Header di luar
            Constraint::Min(0), // Sisanya untuk kontainer
        ])
        .split(area);

    // Header
    render_header(f, outer_chunks[0], ascii_lines, owner);

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
        .style(Style::default().fg(Color::Cyan)).bold();
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
        Line::from(Span::styled(
            "Persamaan Linear Dua Variabel (SPLDV)",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::raw("a1x + b1y = c1")),
        Line::from(Span::raw("a2x + b2y = c2")),
    ]))
    .alignment(Alignment::Center);
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
    for (j, chunk) in [&row1, &row2].iter().enumerate() {
        for (k, area) in chunk.iter().enumerate() {
            let idx = j * 3 + k;
            let input = Paragraph::new(Line::from(vec![
                Span::raw(format!("{}: ", labels[idx])),
                Span::raw(&inputs[idx]),
            ]))
            .block(
                Block::default().borders(Borders::ALL).border_style(if idx == selected {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Blue)
                }),
            )
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

/// Renders the SPLSV input form UI
fn render_input_splsv_ui(
    f: &mut Frame,
    area: Rect,
    inputs: &[String; 2],
    selected: usize,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
) {
    let outer_block = Block::default().title("Form SPLSV").borders(Borders::ALL);
    f.render_widget(outer_block, area);

    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(8), // Header di luar
            Constraint::Min(0), // Sisanya untuk kontainer
        ])
        .split(area);

    // Header
    render_header(f, outer_chunks[0], ascii_lines, owner);

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
        .style(Style::default().fg(Color::Cyan).bold());
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
        Line::from(Span::styled(
            "Persamaan Linear Satu Variabel (SPLSV)",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::raw("ax + b = 0")),
    ]))
    .alignment(Alignment::Center);
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
        .block(
            Block::default().borders(Borders::ALL).border_style(if i == selected {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::Blue).bold()
            }),
        )
        .alignment(Alignment::Left)
        .style(if i == selected {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::White).bold()
        });
        f.render_widget(input, *area);
    }
}

/// Renders the result UI
fn render_result_ui(
    f: &mut Frame,
    area: Rect,
    result_text: &str,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(8), // Header
            Constraint::Min(0), // Kontainer isi hasil
        ])
        .split(area);

    render_header(f, outer_chunks[0], ascii_lines, owner);

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
        .style(Style::default().fg(Color::White).bold());
    let inner_area = container_block.inner(outer_chunks[1]);
    f.render_widget(container_block, outer_chunks[1]);

    let mut lines: Vec<Line> = vec![Line::from(vec![Span::styled(
        "Hasil:",
        Style::default().fg(Color::Green).bold(),
    )])];
    lines.extend(result_text.lines().map(|line| Line::raw(line.to_string())));

    let result = Paragraph::new(Text::from(lines))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(result, inner_area);
}
