/// Errors for Redtime
/// ==================
/// Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// August 2023

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RedtimeError {
    ProjectNotFound(String),
}

impl fmt::Display for RedtimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectNotFound(name) => write!(f, "Project {name} not found")
        }
    }
}

impl Error for RedtimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
