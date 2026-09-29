mod lab;

pub fn router() -> Result<::topcoat::router::Router, Box<dyn std::error::Error + Send + Sync>> {
    use ::topcoat::asset::RouterBuilderAssetExt as _;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;
    use ::topcoat::runtime::RouterBuilderRuntimeExt as _;

    Ok(::topcoat::router::module_router!()
        .discover()
        .assets(::topcoat::asset::AssetBundle::load()?)
        .runtime()
        .build())
}

#[::topcoat::router::page]
async fn redirect() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::redirect("/lab/iro").into())
}
