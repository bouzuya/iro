mod lab;

pub fn router() -> ::topcoat::router::Router {
    use ::topcoat::asset::RouterBuilderAssetExt as _;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;
    use ::topcoat::runtime::RouterBuilderRuntimeExt as _;

    ::topcoat::router::module_router!()
        .discover()
        .assets(::topcoat::asset::AssetBundle::load().unwrap())
        .runtime()
        .build()
}

// async fn redirect(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
#[::topcoat::router::page]
async fn redirect() -> ::topcoat::Result<()> {
    // -> ::topcoat::Result<impl ::topcoat::view::View> {
    Err(::topcoat::router::error::redirect("/lab/iro").into())
}
