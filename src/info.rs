/// Contain various project infos
/// =============================
/// Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// August 2023

use redmine_api::api::{
    issue_statuses::IssueStatus,
    issue_categories::IssueCategory,
    enumerations::TimeEntryActivity,
};

use super::errors::RedtimeError;
use super::helper::RedmineOps;

#[derive(Debug)]
pub struct RedmineInfos {
    pub project_id: u64,

    pub issue_statuses: Vec<IssueStatus>,
    pub activities: Vec<TimeEntryActivity>,
    pub categories: Vec<IssueCategory>,
}

impl RedmineInfos {
    pub fn fetch(client: &RedmineOps, project_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let project_id     = client.fetch_project_id(project_name)?.ok_or(RedtimeError::ProjectNotFound(String::from(project_name)))?;
        let issue_statuses = client.fetch_issue_statuses()?;
        let activities     = client.fetch_activities()?;
        let categories     = client.fetch_categories(project_id)?;


        Ok(Self {
            project_id: project_id,
            issue_statuses: issue_statuses,
            activities: activities,
            categories: categories,
        })
    }

    pub fn find_status_id(&self, name: &str) -> Option<u64> {
        self.issue_statuses.iter().find(|&x| x.name == name).map(|x| x.id)
    }

    pub fn find_activity_id(&self, name: &str) -> Option<u64> {
        self.activities.iter().find(|&x| x.name == name).map(|x| x.id)
    }

    pub fn find_category_id(&self, name: &str) -> Option<u64> {
        self.categories.iter().find(|&x| x.name == name).map(|x| x.id)
    }
}
