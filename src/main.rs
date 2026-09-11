#[tokio::main]
async fn main() {
    use ::topcoat::asset::RouterBuilderAssetExt as _;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;
    use ::topcoat::runtime::RouterBuilderRuntimeExt as _;

    ::topcoat::start(
        ::topcoat::router::Router::builder()
            .discover()
            .assets(::topcoat::asset::AssetBundle::load().unwrap())
            .runtime()
            .build(),
    )
    .await
    .unwrap();
}

#[::topcoat::router::page("/")]
async fn home(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let color = ::topcoat::runtime::signal(cx, || "#4e6a41".to_string());
    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"iro"</title>
                ::topcoat::dev::script()
                ::topcoat::runtime::script()
            </head>
            <body>
                <input
                    @input=$(|e: ::topcoat::runtime::Event| {
                        color.set(e.target.value)
                    })
                    type="color"
                    :value=$(color.get())
                />
                <span>$(color.get())</span>
            </body>
        </html>
    })
}
