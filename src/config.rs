/// Configuration scheme for redtime tool
/// =====================================
/// Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// August 2023

use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct RedtimeConfigGeneral {
    pub project_identifier: String,
    pub add_work_hours: f64,
    pub logfile: String,
}

#[derive(Debug,Deserialize)]
pub struct RedtimeConfigStatus {
    pub new: String,
    pub working: String,
}

#[derive(Debug,Deserialize)]
pub struct RedtimeConfig {
    pub general: RedtimeConfigGeneral,
    pub status: RedtimeConfigStatus,
}
