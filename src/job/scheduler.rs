use crate::job::chatter;
use tokio::sync::OnceCell;
use tokio_cron_scheduler::JobScheduler;

pub async fn singleton_scheduler() -> JobScheduler {
    static INSTANCE: OnceCell<JobScheduler> = OnceCell::const_new();
    let instance = INSTANCE
        .get_or_init(async || {
            let scheduler = JobScheduler::new().await.unwrap();
            scheduler.start().await.unwrap();
            scheduler
        })
        .await;
    instance.clone()
}

pub async fn launch_job() {
    tokio::join!(chatter::run(),);
}
