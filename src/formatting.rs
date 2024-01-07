//! Formatting convenienced, tucked to the side.

use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::*;

use crate::{Args, Command};

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mcm-meta-helper ")?;
        if self.verbose {
            write!(f, "--verbose")?;
        }
        if self.quiet {
            write!(f, "--quiet")?;
        }
        if self.moddir.as_str() != "." {
            write!(f, "--moddir '{}'", self.moddir)?;
        }
        write!(f, " {}", self.cmd)
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Check { opts } => {
                if opts.all {
                    write!(f, "check --all")
                } else if let Some(lang) = &opts.language {
                    write!(f, "check --language {lang}")
                } else {
                    write!(f, "check")
                }
            }
            Command::Update => write!(f, "update"),
            Command::Validate => write!(f, "validate"),
        }
    }
}

/// Print any array of things that can be stringified in a grid
/// in the available terminal space.
pub fn print_in_grid(items: &Vec<impl ToString>, level: log::Level) {
    let width = if let Some((Width(w), Height(_h))) = terminal_size() {
        w - 2
    } else {
        72
    };

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::LeftToRight,
    });
    for item in items {
        grid.add(Cell::from(item.to_string()));
    }

    if let Some(g) = grid.fit_into_width(width.into()) {
        log::log!(level, "{}", g);
    } else {
        log::log!(level, "{}", grid.fit_into_columns(2));
    }
}
