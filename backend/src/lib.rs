use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_tree::HierarchicalLayer;

pub mod db;

pub fn register_tracing() {
    let subscriber = Registry::default()
        .with(HierarchicalLayer::new(2))
        .with(EnvFilter::from_default_env());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
