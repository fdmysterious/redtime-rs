/// Helper stuff for Redmine access
/// ===============================
/// Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// August 2023

use redmine_api::api::Redmine;

use redmine_api::api::projects::{ListProjects, Project, ProjectsWrapper};
use redmine_api::api::issues::{Issue, IssuesWrapper, ListIssues, AssigneeFilter, StatusFilter, UpdateIssue};
use redmine_api::api::issue_statuses::{IssueStatus, IssueStatusesWrapper, ListIssueStatuses};
use redmine_api::api::issue_categories::{IssueCategory, IssueCategoriesWrapper, ListIssueCategories};
use redmine_api::api::enumerations::{TimeEntryActivity, ListTimeEntryActivities, TimeEntryActivitiesWrapper};

use redmine_api::api::time_entries::{CreateTimeEntry};

pub struct RedmineOps {
    client: Redmine,
}

impl RedmineOps {
    pub fn new(client: Redmine) -> Self {
        Self {
            client: client
        }
    }

    pub fn fetch_project_id(&self, name: &str) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        let endpoint = ListProjects::builder().build()?;
        let ProjectsWrapper { projects } =
            self.client.json_response_body::<_, ProjectsWrapper<Project>>(&endpoint)?;
        let pid = projects.iter().find(|&proj| proj.identifier == name).map(|x| x.id);
        Ok(pid)
    }

    pub fn fetch_issue_statuses(&self) -> Result<Vec<IssueStatus>, Box<dyn std::error::Error>> {
        let endpoint = ListIssueStatuses::builder().build()?;
        let IssueStatusesWrapper { issue_statuses } =
            self.client.json_response_body::<_, IssueStatusesWrapper<IssueStatus>>(&endpoint)?;
        Ok(issue_statuses)
    }

    pub fn fetch_activities(&self) -> Result<Vec<TimeEntryActivity>, Box<dyn std::error::Error>> {
        let endpoint = ListTimeEntryActivities::builder().build()?;
        let TimeEntryActivitiesWrapper { time_entry_activities } =
            self.client.json_response_body::<_, TimeEntryActivitiesWrapper<TimeEntryActivity>>(&endpoint)?;
        Ok(time_entry_activities)
    }

    pub fn fetch_categories(&self, project_id: u64) -> Result<Vec<IssueCategory>, Box<dyn std::error::Error>> {
        let endpoint = ListIssueCategories::builder().project_id_or_name(project_id.to_string()).build()?;
        let IssueCategoriesWrapper { issue_categories } =
            self.client.json_response_body::<_, IssueCategoriesWrapper<IssueCategory>>(&endpoint)?;
        Ok(issue_categories)
    }

    /// Fetches ongoing issues for the current user
    pub fn fetch_ongoing_issues(&self, project_id: u64) -> Result<Vec<Issue>, Box<dyn std::error::Error>> {
        let endpoint = ListIssues::builder()
            .project_id(vec![project_id])
            .assignee(AssigneeFilter::Me)
            .status_id(StatusFilter::Open)
            .build()?
        ;

        let IssuesWrapper { issues } =
            self.client.json_response_body::<_, IssuesWrapper<Issue>>(&endpoint)?;
        Ok(issues)
    }

    /// Changes the status of the requested issue
    pub fn update_issue_status(&self, issue_id: u64, new_status_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = UpdateIssue::builder()
            .id(issue_id)
            .status_id(new_status_id)
            .build()?
        ;
        
        self.client.ignore_response_body::<_>(&endpoint)?;
        Ok(())
    }

    /// Adds a time tracking entry to an issue
    pub fn add_time_entry(&self,
        issue_id: u64,
        hours: f64,
        activity_id: u64,
        comment: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = CreateTimeEntry::builder()
            .issue_id(issue_id)
            .hours(hours)
            .activity_id(activity_id)
            .comments(String::from(comment).into())
            .build()?
        ;

        self.client.ignore_response_body::<_>(&endpoint)?;
        Ok(())
    }
}
