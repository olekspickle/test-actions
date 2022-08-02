use crate::defs::{JobInfo, RunInfo};
use octocrab::{models::RunId, Octocrab, Result};
use rocket::{Build, Rocket};
use structopt::StructOpt;

const NAME: &str = "gh-rs";

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = NAME)]
pub struct AppOptions {
    /// Choose a name for your app
    #[structopt(long, short = "n", env, default_value = NAME)]
    pub app_name: String,

    // Github token authorized to do what you want to do.
    #[structopt(long, env)]
    pub token: String,

    // User name, usually it's your GitHub handle
    #[structopt(long, short, env, default_value = NAME)]
    pub gh_user: String,

    // Repository you want to analyze
    #[structopt(long, short, env, default_value = NAME)]
    pub repo_name: String,

    /// App id. Override for more than one instance usage with JWT tokens
    #[structopt(long, env, conflicts_with = "token")]
    pub app_id: Option<u64>,

    /// App private key. Override for more than one instance usage with JWT tokens
    #[structopt(long, env, conflicts_with = "token")]
    pub app_private_key: Option<String>,
}

#[derive(Debug)]
pub struct App {
    pub gh: Octocrab,
    // pub rocket: Option<Rocket<Build>>,
    pub opts: AppOptions,
}

impl App {
    pub fn new(gh: Octocrab, opts: AppOptions) -> Self {
        Self { gh, opts }
    }

    // pub fn rocket(&self) -> Rocket<Build> {
    //     self.rocket.clone().unwrap()
    // }

    pub async fn get_jobs(&self, api: Octocrab, id: RunId) -> Result<Vec<JobInfo>> {
        let wf = api.workflows(&self.opts.gh_user, &self.opts.repo_name);
        let mut page = wf
            .list_jobs(id.clone())
            .per_page(50)
            .page(1u32)
            .send()
            .await
            .unwrap();
        let jobs: Vec<JobInfo> = page.take_items().into_iter().map(JobInfo::from).collect();

        Ok(jobs)
    }

    pub async fn get_runs(&self, api: Octocrab) -> Result<Vec<RunInfo>> {
        let wf = api.workflows(&self.opts.gh_user, &self.opts.repo_name);
        let mut page = wf
            .list_all_runs()
            .per_page(50)
            .page(1u32)
            .send()
            .await
            .unwrap();
        let runs: Vec<RunInfo> = page.take_items().into_iter().map(RunInfo::from).collect();

        Ok(runs)
    }
}