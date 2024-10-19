use tracing::debug;

#[tracing::instrument]
pub async fn do_migration() {
    debug!("Migrating...");
}
