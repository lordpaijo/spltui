use crate::app::{App, AppState};
use crate::themes::gruvbox::*;
use log::debug;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

// Color mapping function
pub fn get_theme_color(color_name: &str, theme: &str) -> Color {
    match theme {
        "dark" => match color_name {
            "orange" => GruvboxDark::ORANGE,
            "yellow" => GruvboxDark::YELLOW,
            "green" => GruvboxDark::GREEN,
            "blue" => GruvboxDark::BLUE,
            "cyan" => GruvboxDark::AQUA,
            "red" => GruvboxDark::RED,
            "gray" => GruvboxDark::GRAY_ALT,
            "fg" => GruvboxDark::FG,
            _ => GruvboxDark::BG,
        },
        "light" => match color_name {
            "orange" => GruvboxLight::ORANGE,
            "yellow" => GruvboxLight::YELLOW,
            "green" => GruvboxLight::GREEN,
            "blue" => GruvboxLight::BLUE,
            "cyan" => GruvboxLight::AQUA,
            "red" => GruvboxLight::RED,
            "gray" => GruvboxLight::GRAY_ALT,
            "fg" => GruvboxLight::FG,
            _ => GruvboxLight::BG,
        },
        _ => Color::White, // Fallback
    }
}

pub fn draw(f: &mut Frame, app: &App, theme_mode: String) {
    let ascii_lines = create_ascii_header(&theme_mode);
    let owner = create_owner_line(&theme_mode);

    // Pass the current theme mode to all rendering functions
    match &app.state {
        AppState::Menu => render_menu_ui(f, f.area(), &ascii_lines, &owner, &theme_mode),
        AppState::InputSPLDV(inputs, selected) => render_input_spldv_ui(
            f,
            f.area(),
            inputs,
            *selected,
            &ascii_lines,
            &owner,
            &theme_mode,
        ),
        AppState::InputSPLSV(inputs, selected) => render_input_splsv_ui(
            f,
            f.area(),
            inputs,
            *selected,
            &ascii_lines,
            &owner,
            &theme_mode,
        ),
        AppState::Result(result) => {
            render_result_ui(f, f.area(), result, &ascii_lines, &owner, &theme_mode)
        }
        AppState::Exit => {}
    }
}

pub fn draw_verbose(f: &mut Frame, app: &App, theme_mode: String) {
    match theme_mode.to_lowercase().as_str() {
        "dark" => debug!("Theme: Dark"),
        "light" => debug!("Theme: Light"),
        _ => {}
    }
    debug!("Drawing UI.");
    let ascii_lines = create_ascii_header(&theme_mode);
    let owner = create_owner_line(&theme_mode);

    match &app.state {
        AppState::Menu => {
            debug!("Rendering: Menu.");
            render_menu_ui(f, f.area(), &ascii_lines, &owner, &theme_mode)
        }
        AppState::InputSPLSV(inputs, selected) => {
            debug!("Rendering: SPLSV Input Form.");
            render_input_splsv_ui(
                f,
                f.area(),
                inputs,
                *selected,
                &ascii_lines,
                &owner,
                &theme_mode,
            )
        }
        AppState::InputSPLDV(inputs, selected) => {
            debug!("Rendering: SPLDV Input Form.");
            render_input_spldv_ui(
                f,
                f.area(),
                inputs,
                *selected,
                &ascii_lines,
                &owner,
                &theme_mode,
            )
        }
        AppState::Result(result) => {
            debug!("Rendering: Result.");
            render_result_ui(f, f.area(), result, &ascii_lines, &owner, &theme_mode)
        }
        AppState::Exit => {}
    }
}

fn create_ascii_header(theme: &str) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(
                "░██████╗██████╗░██╗░░░░░",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "████████╗██╗░░░██╗██╗",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "██╔════╝██╔══██╗██║░░░░░",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "╚══██╔══╝██║░░░██║██║",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "╚█████╗░██████╔╝██║░░░░░",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "░░░██║░░░██║░░░██║██║",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "░╚═══██╗██╔═══╝░██║░░░░░",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "░░░██║░░░██║░░░██║██║",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "██████╔╝██║░░░░░███████╗",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "░░░██║░░░╚██████╔╝██║",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "╚═════╝░╚═╝░░░░░╚══════╝",
                Style::default().fg(get_theme_color("yellow", theme)),
            ),
            Span::styled(
                "░░░╚═╝░░░░╚═════╝░╚═╝",
                Style::default().fg(get_theme_color("green", theme)),
            ),
        ]),
    ]
}

fn create_owner_line(theme: &str) -> Line<'static> {
    Line::styled(
        format!(
            " {} (2025) {} v{} ",
            "LordPaijo",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
        Style::default().fg(get_theme_color("cyan", theme)).bold(),
    )
}

fn render_header(
    f: &mut Frame,
    area: Rect,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
    theme: &str,
) {
    let header = Paragraph::new(Text::from(ascii_lines.to_vec()))
        .style(Style::default().bold())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().bold())
                .bg(get_theme_color("bg", theme))
                .fg(get_theme_color("orange", theme))
                .title(" Header ")
                .title_bottom(owner.clone().centered()),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(header, area);
}

fn render_menu_ui(
    f: &mut Frame,
    area: Rect,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
    theme: &str,
) {
    // Layout utama: Header dan Konten
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(8), // Header
            Constraint::Min(0), // Konten
        ])
        .split(area);

    // Header
    render_header(f, chunks[0], ascii_lines, owner, theme);

    let instructions = Line::from(vec![
        Span::styled(
            " [1/2] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Choose Mode",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            " [Q] ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
        Span::styled(
            "Quit ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
    ]);

    let block = Block::bordered()
        .title_bottom(instructions.centered())
        .border_set(border::THICK)
        .border_style(Style::default().fg(get_theme_color("cyan", theme)));

    // Menu utama
    let menu = Paragraph::new(Line::from(vec![
        Span::raw("\n"),
        Span::styled(
            "[1] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "SPLSV\n",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "        [2] ",
            Style::default().fg(get_theme_color("green", theme)).bold(),
        ),
        Span::styled(
            "SPLDV\n",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(
        block
            .borders(Borders::ALL)
            .bg(get_theme_color("bg", theme))
            .title(" Menu ")
            .bold(),
    );

    f.render_widget(menu, chunks[1]);
}

fn render_input_spldv_ui(
    f: &mut Frame,
    area: Rect,
    inputs: &[String; 6],
    selected: usize,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
    theme: &str,
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
    render_header(f, outer_chunks[0], ascii_lines, owner, theme);

    // Buat blok kontainer (dengan border)
    let instructions = Line::from(vec![
        Span::styled(
            " [Esc] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Menu ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[←/→] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Left/Right ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[Enter] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Submit ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[Q] ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
        Span::styled(
            "Quit ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
    ]);

    let container_block = Block::default()
        .title_bottom(instructions.centered())
        .title(" Input SPLDV ")
        .borders(Borders::ALL)
        .style(
            Style::default()
                .bg(get_theme_color("bg", theme))
                .fg(get_theme_color("cyan", theme))
                .bold(),
        );
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
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(if idx == selected {
                        Style::default().fg(get_theme_color("yellow", theme))
                    } else {
                        Style::default().fg(get_theme_color("blue", theme))
                    }),
            )
            .alignment(Alignment::Left)
            .style(if idx == selected {
                Style::default().fg(get_theme_color("yellow", theme))
            } else {
                Style::default().fg(get_theme_color("fg", theme))
            });
            f.render_widget(input, *area);
        }
    }
}

fn render_input_splsv_ui(
    f: &mut Frame,
    area: Rect,
    inputs: &[String; 2],
    selected: usize,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
    theme: &str,
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
    render_header(f, outer_chunks[0], ascii_lines, owner, theme);

    // Buat blok kontainer (dengan border)
    let instructions = Line::from(vec![
        Span::styled(
            " [Esc] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Menu ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[←/→] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Left/Right ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[Enter] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Submit ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[Q] ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
        Span::styled(
            "Quit ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
    ]);

    let container_block = Block::default()
        .title_bottom(instructions.centered())
        .title(" Input SPLSV ")
        .borders(Borders::ALL)
        .style(
            Style::default()
                .bg(get_theme_color("bg", theme))
                .fg(get_theme_color("cyan", theme))
                .bold(),
        );
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
            Block::default()
                .borders(Borders::ALL)
                .border_style(if i == selected {
                    Style::default().fg(get_theme_color("yellow", theme)).bold()
                } else {
                    Style::default().fg(get_theme_color("blue", theme)).bold()
                }),
        )
        .alignment(Alignment::Left)
        .style(if i == selected {
            Style::default().fg(get_theme_color("yellow", theme)).bold()
        } else {
            Style::default().fg(get_theme_color("fg", theme)).bold()
        });
        f.render_widget(input, *area);
    }
}

fn render_result_ui(
    f: &mut Frame,
    area: Rect,
    result_text: &str,
    ascii_lines: &Vec<Line<'_>>,
    owner: &Line<'_>,
    theme: &str,
) {
    let outer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(8), // Header
            Constraint::Min(0), // Kontainer isi hasil
        ])
        .split(area);

    render_header(f, outer_chunks[0], ascii_lines, owner, theme);

    let instructions = Line::from(vec![
        Span::styled(
            " [Esc] ",
            Style::default().fg(get_theme_color("blue", theme)).bold(),
        ),
        Span::styled(
            "Menu ",
            Style::default().fg(get_theme_color("fg", theme)).bold(),
        ),
        Span::styled(
            "[Q] ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
        Span::styled(
            "Quit ",
            Style::default().fg(get_theme_color("red", theme)).bold(),
        ),
    ]);

    let container_block = Block::default()
        .title(" Hasil Perhitungan ")
        .title_bottom(instructions.centered())
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .border_style(Style::default().fg(get_theme_color("cyan", theme)))
        .style(
            Style::default()
                .bg(get_theme_color("bg", theme))
                .fg(get_theme_color("fg", theme))
                .bold(),
        );
    let inner_area = container_block.inner(outer_chunks[1]);
    f.render_widget(container_block, outer_chunks[1]);

    let mut lines: Vec<Line> = vec![Line::from(vec![Span::styled(
        "Hasil:",
        Style::default().fg(get_theme_color("green", theme)).bold(),
    )])];
    lines.extend(result_text.lines().map(|line| Line::raw(line.to_string())));

    let result = Paragraph::new(Text::from(lines))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(result, inner_area);
}
