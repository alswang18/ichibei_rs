use ichibei_rs::configuration::get_configuration;
use ichibei_rs::startup::Application;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration."); // Use a different database for each test case
                                                                                 // Use a random OS port
        c.application.port = 0;
        c
    };

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    let test_app = TestApp {
        address,
        port: application_port,
        api_client: client,
    };

    test_app
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
