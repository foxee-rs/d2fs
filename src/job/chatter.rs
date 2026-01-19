use crate::job::scheduler::singleton_scheduler;
use tokio_cron_scheduler::Job;
use tracing::trace;

pub(super) async fn run() {
    let scheduler = singleton_scheduler().await;
    scheduler
        .add(
            Job::new("*/3 * * * * *", |_uuid, _l| {
                sync_task();
            })
            .unwrap(),
        )
        .await
        .unwrap();

    scheduler
        .add(Job::new_async("*/5 * * * * *", |_uuid, mut _l| Box::pin(async_task())).unwrap())
        .await
        .unwrap();
}

fn sync_task() {
    trace!("chatter.sync_task tick");
}

async fn async_task() {
    trace!("chatter.async_task tick");
}
