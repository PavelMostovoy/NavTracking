use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // our router

    // which calls one of these handlers
    async fn root() -> &'static str {
         "Hello, World!"}
    async fn get_foo() -> &'static str  {"foo"}
    async fn post_foo()-> &'static str {"post foo"}
    async fn foo_bar() -> &'static str {"get bar"}
}