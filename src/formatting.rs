//! Formatting conveniences, tucked to the side.

use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::*;

use crate::{Args, Command};

/// Make a gridded string from any array of things that can be stringified
/// in the available terminal space.
pub fn grid_string(items: &Vec<impl ToString>, space: u16) -> String {
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

    if let Some(g) = grid.fit_into_width((width - space).into()) {
        g.to_string()
    } else {
        grid.fit_into_columns(2).to_string()
    }
}

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
            Command::Check { ref language, .. } => {
                if (*language).as_str() == "all" {
                    write!(f, "check --all")
                } else {
                    write!(f, "check --language {language}")
                }
            }
            Command::Copy { ref language } => write!(f, "copy {language}"),
            Command::Update => write!(f, "update"),
            Command::Validate => write!(f, "validate"),
        }
    }
}
