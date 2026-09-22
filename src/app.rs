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

#[::topcoat::router::page]
async fn redirect() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::redirect("/lab/iro").into())
}
