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
    // bouzuya-green: #4e6a41 rgb(78, 106, 65)
    let red = ::topcoat::runtime::signal(cx, || 78.0);
    let green = ::topcoat::runtime::signal(cx, || 106.0);
    let blue = ::topcoat::runtime::signal(cx, || 65.0);

    let color = format!(
        "#{:02X}{:02X}{:02X}",
        red.get() as u8,
        green.get() as u8,
        blue.get() as u8
    );

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
                    @input=$(async |e: ::topcoat::runtime::Event| {
                        let s = e.target.value.to_owned();
                        let r = color_to_rgb(s.clone(), 0.0).await;
                        let g = color_to_rgb(s.clone(), 1.0).await;
                        let b = color_to_rgb(s.clone(), 2.0).await;
                        red.set(r);
                        green.set(g);
                        blue.set(b);
                    })
                    type="color"
                    value=(color.clone())
                />
                <span>(color)</span>
                <div>
                    <span>"RGB (R, G, B)"</span>
                    <label>
                        <span>"R"</span>
                        <input
                            @input=$(async |e: ::topcoat::runtime::Event| {
                                let s = e.target.value.to_owned();
                                // TODO: convert the string to a float without procedure
                                let n = str_to_f64(s).await;
                                red.set(n);
                            })
                            max="255"
                            min="0"
                            type="number"
                            :value=$(red.get())
                        />
                    </label>
                    <label>
                        <span>"G"</span>
                        <input
                            @input=$(async |e: ::topcoat::runtime::Event| {
                                let s = e.target.value.to_owned();
                                // TODO: convert the string to a float without procedure
                                let n = str_to_f64(s).await;
                                green.set(n);
                            })
                            max="255"
                            min="0"
                            type="number"
                            :value=$(green.get())
                        />
                    </label>
                    <label>
                        <span>"B"</span>
                        <input
                            @input=$(async |e: ::topcoat::runtime::Event| {
                                let s = e.target.value.to_owned();
                                // TODO: convert the string to a float without procedure
                                let n = str_to_f64(s).await;
                                blue.set(n);
                            })
                            max="255"
                            min="0"
                            type="number"
                            :value=$(blue.get())
                        />
                    </label>
                </div>
            </body>
        </html>
    })
}

#[::topcoat::runtime::shard]
async fn hex_input(
    blue: ::topcoat::runtime::Signal<f64>,
) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let value = blue.get();
    let value = format!("{:02X}", value as u8);
    // Ok(::topcoat::view::view! { <input maxlength="2" type="text" value=(value) /> })
    Ok(::topcoat::view::view! {
        <input
            @input=$(async |e: ::topcoat::runtime::Event| {
                let s = e.target.value.to_owned();
                // TODO: convert the string to a float without procedure
                let n = hex_str_to_f64(s).await;
                blue.set(n);
            })
            maxlength="2"
            type="text"
            value=(value)
        />
    })
}

#[::topcoat::runtime::procedure]
async fn color_to_rgb(value: String, f: f64) -> ::topcoat::Result<f64> {
    let value = value.strip_prefix('#').unwrap_or("000000");
    if value.len() != 6
        || value.chars().any(|c| !c.is_ascii_hexdigit())
        || !(f == 0.0 || f == 1.0 || f == 2.0)
    {
        return Ok(0.0);
    }
    Ok(u8::from_str_radix(
        &value[match f {
            0.0 => 0..2,
            1.0 => 2..4,
            2.0 => 4..6,
            _ => unreachable!(),
        }],
        16,
    )
    .unwrap_or(0) as f64)
}

#[::topcoat::runtime::procedure]
async fn hex_str_to_f64(value: String) -> ::topcoat::Result<f64> {
    Ok(u8::from_str_radix(&value, 16).unwrap_or(0) as f64)
}

#[::topcoat::runtime::procedure]
async fn str_to_f64(value: String) -> ::topcoat::Result<f64> {
    Ok(value.parse::<f64>().unwrap_or(0.0))
}
