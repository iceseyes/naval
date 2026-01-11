use crate::engine::game::Game;
use crate::{
    engine::{
        fleet::{Fleet, Ship, ShipKind, ShipOrientation},
        grid::Grid,
        player::Player,
    },
    tui::{
        state::StateModel,
        widgets::grid::{GridModel, Layer},
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Buffer, Constraint, Direction, Layout, Line, Rect, Span, Stylize, Text, Widget},
    symbols::border,
    widgets::{Block, Paragraph},
};

/// Model for the setup state.
///
/// During the setup phase, the player positions their ships on the grid. Every type of ship
/// must be placed. The model tracks these placements, and once all ships are in position,
/// the setup phase is complete.
pub struct SetupStateModel {
    deploy_grid: GridModel,
    current_kind: Option<ShipKind>,
    current_orientation: ShipOrientation,
    ships: Vec<Ship>,
    kind_iter: std::slice::Iter<'static, ShipKind>,
}

impl SetupStateModel {
    const SHIP_KINDS: [ShipKind; 5] = [
        ShipKind::AircraftCarrier,
        ShipKind::Battleship,
        ShipKind::Cruiser,
        ShipKind::Submarine,
        ShipKind::Destroyer,
    ];

    fn update_grid(&mut self) {
        self.deploy_grid.pop_layer();
        if let Some(ref kind) = self.current_kind
            && let Some(ship) = kind.ship(
                *self.deploy_grid.cursor().unwrap(),
                self.current_orientation,
            )
        {
            self.deploy_grid.push_layer(Layer::Ship(ship));
        }
    }
}

impl Default for SetupStateModel {
    /// Creates a new setup state with an empty deployment grid and no placed ships.
    /// The first ship to be placed is the Aircraft Carrier.
    fn default() -> Self {
        let mut kind_iter = Self::SHIP_KINDS.iter();
        let mut deploy_grid = GridModel::new(Grid::default());
        deploy_grid.enable_cursor(); // Ensures the cursor is enabled

        let mut model = Self {
            deploy_grid,
            current_kind: kind_iter.next().cloned(),
            current_orientation: ShipOrientation::Horizontal,
            ships: Vec::new(),
            kind_iter,
        };

        model.update_grid();

        model
    }
}

impl StateModel for SetupStateModel {
    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.deploy_grid.move_cursor(|c| c.move_left()),
            KeyCode::Right => self.deploy_grid.move_cursor(|c| c.move_right()),
            KeyCode::Up => self.deploy_grid.move_cursor(|c| c.move_up()),
            KeyCode::Down => self.deploy_grid.move_cursor(|c| c.move_down()),
            KeyCode::Enter => {
                if let Some(ref kind) = self.current_kind
                    && let Some(ship) = kind.ship(
                        *self.deploy_grid.cursor().unwrap(),
                        self.current_orientation,
                    )
                    && self.ships.iter().all(|s| !ship.is_overlapping(s))
                {
                    self.deploy_grid.add_ship(&ship);
                    self.ships.push(ship.clone());
                    self.current_kind = self.kind_iter.next().cloned();
                }
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.current_orientation = ShipOrientation::Horizontal
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                self.current_orientation = ShipOrientation::Vertical
            }
            _ => {}
        }

        self.update_grid();
    }

    fn update(&mut self, game: &mut Game) {
        if self.current_kind.is_none() {
            let human = Player::new("player 1", Fleet::new(self.ships.as_slice()).unwrap());
            game.set_human_player(human);
        }
    }

    fn widget(&self) -> impl Widget {
        SetupWidget::new(self)
    }
}

/// Widget for the setup state.
pub struct SetupWidget<'state>(&'state SetupStateModel);

impl<'state> SetupWidget<'state> {
    /// Builds a new widget upon the given state.
    pub fn new(state: &'state SetupStateModel) -> Self {
        Self(state)
    }
}

impl<'state> Widget for SetupWidget<'state> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let deploy_grid = &self.0.deploy_grid;
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let deploy_block = Block::bordered()
            .title(Line::from("Deploy your fleet".bold()))
            .border_set(border::THICK);

        deploy_grid
            .widget()
            .render(deploy_block.inner(layout[0]), buf);

        deploy_block.render(layout[0], buf);

        let notes_block = Block::bordered()
            .title(Line::from("Help".bold()))
            .border_set(border::THICK);

        let help_text = Text::from(vec![
            Line::from("Welcome to Naval - The Battleship Game")
                .red()
                .bold()
                .centered(),
            Line::from(""),
            Line::from("Use:").bold().centered(),
            Line::from("- the arrow keys: to move the ship").centered(),
            Line::from("- h: to put the ship horizontally").centered(),
            Line::from("- v: to put the ship vertically").centered(),
            Line::from("- Enter: to place it.").centered(),
            Line::from(""),
            Line::from(vec![
                Span::raw("Please, place your ").gray(),
                Span::raw(format!("{}", self.0.current_kind.as_ref().unwrap()))
                    .yellow()
                    .bold(),
                Span::raw(" [size: ").gray(),
                Span::raw(format!("{}", self.0.current_kind.as_ref().unwrap().size()))
                    .yellow()
                    .italic(),
                Span::raw("]").gray(),
            ])
            .centered(),
        ]);

        let text = Paragraph::new(help_text).block(notes_block);

        text.render(layout[1], buf);
    }
}
