use crossterm::event::{KeyCode, KeyEvent};
use matematika_rs::sistem::aljabar::*;
use std::time::{Duration, Instant};

#[derive(Clone, PartialEq)]
pub enum AppState {
    Menu,
    InputSPLDV([String; 6], usize),
    InputSPLSV([String; 2], usize),
    Result(String),
    Exit,
}

/// Application state and logic
pub struct App {
    pub state: AppState,
    last_key_time: Instant,
    debounce_duration: Duration,
}

impl App {
    pub fn new(splsv: bool, spldv: bool, hasil: bool) -> Self {
        let state = if splsv {
            println!("Running with [--splsv]");
            AppState::InputSPLSV(std::array::from_fn(|_| "".to_string()), 0)
        } else if spldv {
            AppState::InputSPLDV(std::array::from_fn(|_| "".to_string()), 0)
        } else if hasil {
            AppState::Result(
                "Tidak ada hasil dari flag [--hasil]. Gunakan no-flag atau [--splsv/--spldv]"
                    .to_string(),
            )
        } else {
            println!("Running default.");
            AppState::Menu
        };

        Self {
            state,
            last_key_time: Instant::now(),
            #[cfg(target_os = "windows")]
            debounce_duration: Duration::from_millis(175),
            #[cfg(not(target_os = "windows"))]
            debounce_duration: Duration::from_millis(50),
        }
    }

    pub fn on_key(&mut self, key_event: KeyEvent) {
        let now = Instant::now();
        if now.duration_since(self.last_key_time) < self.debounce_duration {
            return;
        }

        self.last_key_time = now;

        match self.state.clone() {
            AppState::Menu => self.handle_menu_key(key_event.code),
            AppState::InputSPLDV(inputs, selected) => {
                self.handle_spldv_key(key_event.code, inputs, selected)
            }
            AppState::InputSPLSV(inputs, selected) => {
                self.handle_splsv_key(key_event.code, inputs, selected)
            }
            AppState::Result(_) => self.handle_result_key(key_event.code),
            AppState::Exit => {}
        }
    }

    fn handle_menu_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('1') => {
                self.state = AppState::InputSPLSV(std::array::from_fn(|_| "".to_string()), 0)
            }
            KeyCode::Char('2') => {
                self.state = AppState::InputSPLDV(std::array::from_fn(|_| "".to_string()), 0)
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,
            _ => {}
        }
    }

    fn handle_spldv_key(&mut self, key: KeyCode, mut inputs: [String; 6], selected: usize) {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
                inputs[selected].push(c);
                self.state = AppState::InputSPLDV(inputs, selected);
            }
            KeyCode::Backspace => {
                inputs[selected].pop();
                self.state = AppState::InputSPLDV(inputs, selected);
            }
            KeyCode::Left => {
                let new_selected = if selected > 0 { selected - 1 } else { selected };
                self.state = AppState::InputSPLDV(inputs, new_selected);
            }
            KeyCode::Right => {
                let new_selected = if selected < inputs.len() - 1 {
                    selected + 1
                } else {
                    selected
                };
                self.state = AppState::InputSPLDV(inputs, new_selected);
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
        }
    }

    fn handle_splsv_key(&mut self, key: KeyCode, mut inputs: [String; 2], selected: usize) {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
                inputs[selected].push(c);
                self.state = AppState::InputSPLSV(inputs, selected);
            }
            KeyCode::Backspace => {
                inputs[selected].pop();
                self.state = AppState::InputSPLSV(inputs, selected);
            }
            KeyCode::Left => {
                let new_selected = if selected > 0 { selected - 1 } else { selected };
                self.state = AppState::InputSPLSV(inputs, new_selected);
            }
            KeyCode::Right => {
                let new_selected = if selected < inputs.len() - 1 {
                    selected + 1
                } else {
                    selected
                };
                self.state = AppState::InputSPLSV(inputs, new_selected);
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
        }
    }

    fn handle_result_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.state = AppState::Exit,
            KeyCode::Esc => self.state = AppState::Menu,
            _ => {}
        }
    }

    pub fn should_exit(&self) -> bool {
        matches!(self.state, AppState::Exit)
    }
}
