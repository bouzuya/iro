#[tokio::main]
async fn main() {
    use topcoat::router::RouterBuilderDiscoverExt as _;

    ::topcoat::start(::topcoat::router::Router::builder().discover().build())
        .await
        .unwrap();
}

#[::topcoat::router::page("/")]
async fn home() -> ::topcoat::Result<::topcoat::view::View> {
    ::topcoat::view::view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hello world"</title>
                topcoat::dev::script()
            </head>
            <body>hello(name: "World")</body>
        </html>
    }
}

#[::topcoat::view::component]
async fn hello(name: &str) -> ::topcoat::Result<::topcoat::view::View> {
    ::topcoat::view::view! {
        <h1>
            "Hello, "
            (name)
            "!"
        </h1>
    }
}
