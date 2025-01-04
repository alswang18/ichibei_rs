use ichibei_rs::configuration::get_configuration;
use ichibei_rs::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    application.run_until_stopped().await?;
    Ok(())
}
