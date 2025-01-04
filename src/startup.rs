use crate::configuration::Settings;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};

use std::net::TcpListener;
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// Print hell world to return
async fn hello_world() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

// This is the application state that is shared across all routes.
#[derive(Clone)]
pub struct AppState {
    oauth_client: std::sync::Arc<BasicClient>,
}

pub struct Application {
    port: u16,
    server: Server,
    app_state: AppState,
}

impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let google_client_id = ClientId::new(
            std::env::var("GOOGLE_CLIENT_ID")
                .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
        );
        let google_client_secret = ClientSecret::new(
            std::env::var("GOOGLE_CLIENT_SECRET")
                .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        let redirect = RedirectUrl::new(format!(
            "{}/google/auth",
            configuration.oauth2_client.redirect_uri
        ))
        .expect("Invalid redirect URL");

        // Set up the config for the Google OAuth2 process.
        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect);

        let app_state = AppState {
            oauth_client: std::sync::Arc::new(client),
        };

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, app_state.oauth_client.clone()).await?;
        // We "save" the bound port in one of `Application`'s fields
        Ok(Self {
            port,
            server,
            app_state,
        })
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(
    listener: TcpListener,
    oauth2_client: std::sync::Arc<BasicClient>,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(hello_world))
            .route("/health_check", web::get().to(health_check))
            .app_data(oauth2_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
