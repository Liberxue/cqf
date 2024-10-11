use clap::Parser;
use core::models::{OptionParameters, OptionPricingModel};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table, TableState},
    Frame, Terminal,
};
use std::cell::RefCell;
use std::io::{self};
use std::{fs, path::Path};

#[derive(Parser)]
struct Opts {
    #[arg(short, long)]
    s: f64,
    #[arg(short, long)]
    k: f64,
    #[arg(short, long)]
    r: f64,
    #[arg(short = 'm', long)]
    sigma: f64,
    #[arg(short, long)]
    t: f64,
}

#[derive(Clone, PartialEq, Debug)]
struct ModelResults {
    call: f64,
    put: f64,
    delta: f64,
    gamma: f64,
    vega: f64,
    theta: f64,
    rho: f64,
}

struct ModelWrapper {
    name: String,
    model: Box<dyn OptionPricingModel>,
    cache: RefCell<Option<ModelResults>>,
}

impl ModelWrapper {
    fn get_results(&self, params: &OptionParameters) -> ModelResults {
        if let Some(cached) = self.cache.borrow().as_ref() {
            return cached.clone();
        }

        let results = ModelResults {
            call: self.model.call_price(params),
            put: self.model.put_price(params),
            delta: self.model.delta(params),
            gamma: self.model.gamma(params),
            vega: self.model.vega(params),
            theta: self.model.theta(params),
            rho: self.model.rho(params),
        };

        *self.cache.borrow_mut() = Some(results.clone());
        results
    }

    fn invalidate_cache(&self) {
        *self.cache.borrow_mut() = None;
    }
}

struct App {
    models: Vec<ModelWrapper>,
    table_state: TableState,
    params: OptionParameters,
    params_changed: bool,
}

impl App {
    fn new(opts: Opts) -> Self {
        let params = OptionParameters {
            s: opts.s,
            k: opts.k,
            r: opts.r,
            sigma: opts.sigma,
            t: opts.t,
        };
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        App {
            models: load_models(),
            table_state,
            params,
            params_changed: true,
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.models.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.models.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn update_params(&mut self, new_params: OptionParameters) {
        if self.params != new_params {
            self.params = new_params;
            self.params_changed = true;
            for model in &self.models {
                model.invalidate_cache();
            }
        }
    }
}

fn create_model(model_name: &str) -> Option<Box<dyn OptionPricingModel>> {
    match model_name {
        "black_scholes" => Some(Box::new(core::models::BlackScholesModel)),
        "binomial_tree" => Some(Box::new(core::models::BinomialTreeModel::default())),
        "garch" => Some(Box::new(core::models::GarchModel::default())),
        "monte_carlo" => Some(Box::new(core::models::MonteCarloModel {
            simulations: 1000,
            time_steps: 10,
        })),
        _ => None,
    }
}

fn is_valid_model_file(entry: &fs::DirEntry) -> bool {
    entry.path().is_file()
        && entry.path().extension().and_then(|s| s.to_str()) == Some("rs")
        && entry
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s != "mod")
            .unwrap_or(false)
}

fn create_model_wrapper(entry: &fs::DirEntry) -> Option<ModelWrapper> {
    entry
        .path()
        .file_stem()
        .and_then(|s| s.to_str())
        .and_then(|model_name| {
            create_model(model_name).map(|model| ModelWrapper {
                name: model_name.to_string(),
                model,
                cache: RefCell::new(None),
            })
        })
}

fn load_models() -> Vec<ModelWrapper> {
    let model_dir = Path::new("../core/src/models");

    fs::read_dir(model_dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(is_valid_model_file)
                .filter_map(|entry| create_model_wrapper(&entry))
                .collect()
        })
        .unwrap_or_else(|_| Vec::new())
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let opts: Opts = Opts::parse();
    let mut app = App::new(opts);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        if app.params_changed {
            terminal.draw(|f| ui(f, app))?;
            app.params_changed = false;
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
                app.params_changed = true;
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100), Constraint::Ratio(1, 8)])
        .split(f.area());

    let header_cells = [
        "Models", "Call", "Put", "Delta", "Gamma", "Vega", "Theta", "Rho",
    ]
    .iter()
    .map(|h| {
        Cell::from(*h).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    });
    let header = Row::new(header_cells).style(Style::default().bg(Color::Black));
    let rows = app.models.iter().map(|wrapper| {
        let results = wrapper.get_results(&app.params);
        let cells = vec![
            Cell::from(wrapper.name.as_str()),
            Cell::from(format!("{:.4}", results.call)),
            Cell::from(format!("{:.4}", results.put)),
            Cell::from(format!("{:.4}", results.delta)),
            Cell::from(format!("{:.4}", results.gamma)),
            Cell::from(format!("{:.4}", results.vega)),
            Cell::from(format!("{:.4}", results.theta)),
            Cell::from(format!("{:.4}", results.rho)),
        ];
        Row::new(cells)
    });

    let widths = [
        Constraint::Percentage(15),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(12),
        Constraint::Percentage(13),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default())
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
        .highlight_symbol("> ");

    f.render_stateful_widget(table, chunks[0], &mut app.table_state.clone());
}
