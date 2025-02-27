use zero2prod::{
    configuration::get_configuration,
    issue_delivery_worker::run_worker_until_stopped,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let app_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    // make progress concurrently
    tokio::select! {
        _ = app_task => {},
        _ = worker_task => {},
    };

    Ok(())
}
