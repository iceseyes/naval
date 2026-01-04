//! This module contains the UI widgets for the Naval Battle TUI application.
//!
//! Each widget is responsible for rendering a specific part of the application UI. For instance,
//! the grid widget is responsible for rendering the [crate::engine::grid::Grid].
//!
//! The root widget is the [crate::tui::workbench::Workbench], which is responsible for rendering the entire application UI.
//! Basically, the workbench draws the minimal items of the entire application, like the border, title, and instructions, and
//! it takes a *content* to be rendered inside the workbench itself.
//!
pub mod battle;
pub mod grid;
pub mod setup;
pub mod workbench;
