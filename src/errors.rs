/// Errors for Redtime
/// ==================
/// Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// August 2023

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RedtimeError {
    ProjectNotFound(String),

    FetchProjectsError,
    FetchStatusesError,
    FetchActivitiesError,
    FetchCategoriesError,
}

impl fmt::Display for RedtimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectNotFound(name) => write!(f, "Project {name} not found"),
            Self::FetchProjectsError    => write!(f, "Cannot fetch projects"   ),
            Self::FetchStatusesError    => write!(f, "Cannot fetch statuses"   ),
            Self::FetchActivitiesError  => write!(f, "Cannot fetch activities" ),
            Self::FetchCategoriesError  => write!(f, "Cannot fetch categories" ),
        }
    }
}

impl Error for RedtimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
