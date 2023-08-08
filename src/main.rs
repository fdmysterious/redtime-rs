/// Redtime: Dumb tool to time track redmine activities
/// ===================================================
/// Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// August 2023

use redmine_api::api::Redmine;

use std::error::Error;
use std::fmt;

use dotenvy;

use inquire::{Select, Text};

mod errors;
mod config;
mod helper;
mod info;

use helper::RedmineOps;
use info::RedmineInfos;
use config::RedtimeConfig;

use serde::Deserialize;

use std::fs::{
    self,
    OpenOptions
};

use std::io::prelude::*;

use chrono::{Local, Timelike};

use toml;

use std::fmt::Display;

use redmine_api::api::issues::Issue;
use redmine_api::api::enumerations::TimeEntryActivity;

use log::debug;

use env_logger;
use better_panic;

/// Some utility class using some ID and string
#[derive(Debug)]
struct IdChoice {
    id: u64,
    name: String,
}

impl Display for IdChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl From<&Issue> for IdChoice {
    fn from(other: &Issue) -> Self {
        Self {
            id: other.id,
            name: other.subject.clone().unwrap_or(String::from("Unamed issue")),
        }
    }
}

impl From<&TimeEntryActivity> for IdChoice {
    fn from(other: &TimeEntryActivity) -> Self {
        Self {
            id: other.id,
            name: other.name.clone(),
        }
    }
}

fn main() {
    env_logger::init();
    better_panic::install();

    // Load config
    dotenvy::dotenv().unwrap();
    let config: RedtimeConfig = toml::from_str(&fs::read_to_string("redtime.toml").unwrap()).unwrap();

    // Init redmine client
    let redmine = Redmine::from_env().unwrap();
    let redmine = RedmineOps::new(redmine);

    // Load infos from redmine
    let infos       = RedmineInfos::fetch(&redmine, &config.general.project_identifier).unwrap();
    let status_new  = infos.find_status_id(&config.status.new    ).expect("Could not find status ID for indicated new status");
    let status_work = infos.find_status_id(&config.status.working).expect("Could not find status ID for working status"      );

    debug!("Got status_new = {}, status_work={}", status_new, status_work);
    
    // Ask issue -> Only display issues that have the "new" or "working" status
    let issues  = redmine.fetch_ongoing_issues(infos.project_id).expect("Could not fetch issues");
    let choices = issues.iter()
        .filter(|i| (i.status.id == status_new) || (i.status.id == status_work))
        .map(IdChoice::from)
        .collect();

    let chosen  = Select::new(
        "Please choose an issue",
        choices
    ).prompt().unwrap();
    let chosen_issue = issues.iter().find(|&x| x.id == chosen.id).expect("Error finding corresponding issue");

    // Ask type of activity
    let choices = infos.activities.iter().map(IdChoice::from).collect();
    let chosen  = Select::new(
        "Please select a type of activity",
        choices
    ).prompt().unwrap();
    let chosen_activity = infos.activities.iter().find(|&x| x.id == chosen.id).expect("Error finding corresponding activity type");
    
    // Ask comment
    let comment = Text::new("What is the purpose of your work?").prompt().unwrap();

    // Check task status
    if &chosen_issue.status.name == &config.status.new {
        println!("> Task status is \"{}\", changing to \"{}\"", &chosen_issue.status.name, &config.status.working);
        redmine.update_issue_status(chosen_issue.id, status_work).unwrap();
    }

    // Add time tracking activity
    println!("> Add time tracking entry: {} hours, comment: {}", config.general.add_work_hours, &comment);
    redmine.add_time_entry(chosen_issue.id, config.general.add_work_hours, chosen_activity.id, &comment).unwrap();

    // Everything fine, log to log file
    println!("> Log entry to {}", config.general.logfile);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(config.general.logfile)
        .unwrap();

    // TODO Initialize columns if file doesn't exist?
    // Columns: Date, AM/PM, Issue subject, Activity type, hours, comment
    let  now       = Local::now();
    let (is_pm, _) = now.hour12();

    let datestr    = now.format("%Y-%m-%d %H:%M:%S");
    let ampm       = String::from(if is_pm {"PM"} else {"AM"});
    let subject    = chosen_issue.subject.clone().unwrap_or(String::from("Unamed issue"));
    let activity   = chosen_activity.name.clone();
    let hours      = config.general.add_work_hours;

    writeln!(file, "{};{};{};{};{};{}",
        datestr,
        ampm,
        subject,
        activity,
        hours,
        comment
    ).expect("Couldn't write to log CSV file");
}
