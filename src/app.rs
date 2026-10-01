mod lab;

pub fn router() -> Result<::topcoat::router::Router, Box<dyn std::error::Error + Send + Sync>> {
    use ::topcoat::asset::RouterBuilderAssetExt as _;
    use ::topcoat::router::RouterBuilderDirectoryExt as _;
    use ::topcoat::router::RouterBuilderDiscoverExt as _;
    use ::topcoat::runtime::RouterBuilderRuntimeExt as _;

    trait MyRouterBuilderExt: Sized {
        fn assets_with_workaround_for_base_path(
            self,
        ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>;
    }

    impl MyRouterBuilderExt for ::topcoat::router::RouterBuilder {
        fn assets_with_workaround_for_base_path(
            self,
        ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
            let result = if std::env::var("TOPCOAT_DEV_URL").is_ok() {
                self.assets(::topcoat::asset::AssetBundle::load()?)
            } else {
                // workaround for asset base path by using hosted_at and serve_dir combination
                self.assets(::topcoat::asset::AssetConfig::hosted_at(
                    format!(
                        "{}/lab/iro/assets",
                        std::env::var("ORIGIN").map_err(|_| "not set ORIGIN env var")?
                    ),
                    ::topcoat::asset::AssetBundle::load_dir("src/app/lab/iro/assets")?,
                ))
                .serve_dir("/lab/iro/assets/{*file}", "src/app/lab/iro/assets")
            };
            Ok(result)
        }
    }

    Ok(::topcoat::router::module_router!()
        .discover()
        .assets_with_workaround_for_base_path()?
        .runtime()
        .build())
}

#[::topcoat::router::page]
async fn redirect() -> ::topcoat::Result<()> {
    Err(::topcoat::router::error::redirect("/lab/iro").into())
}
