#[tokio::main]
async fn main() {
    let app = ApiRouter::new()
        // Change `route` to `api_route` for the route
        // we'd like to expose in the documentation.
        .api_route("/hello", post(hello_user))
        // We'll serve our generated document here.
        .route("/api.json", get(serve_api));

    let mut api =
        OpenApi { info: Info { description: Some("an example API".to_string()), ..Info::default() }, ..OpenApi::default() };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
        listener,
        app
            // Generate the documentation.
            .finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
