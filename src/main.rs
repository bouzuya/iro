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

#[::topcoat::router::query_params(error = bad_request)]
struct HomeQueryParams {
    c: Option<String>,
}

#[::topcoat::router::page("/")]
async fn home(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let HomeQueryParams { c } = ::topcoat::router::query_params::<HomeQueryParams>(cx)?;
    let c = c.clone().unwrap_or_else(|| "#4e6a41".to_string());

    // bouzuya-green: #4e6a41 rgb(78, 106, 65)
    let red = ::topcoat::runtime::signal(cx, || rgb(&c).0 as f64);
    let green = ::topcoat::runtime::signal(cx, || rgb(&c).1 as f64);
    let blue = ::topcoat::runtime::signal(cx, || rgb(&c).2 as f64);

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
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"iro"</title>
                <link href=(::topcoat::asset::asset!("./index.css")) rel="stylesheet" />
                ::topcoat::dev::script()
                ::topcoat::runtime::script()
            </head>
            <body>
                <div class="section">
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
                    <div>
                        <h2>"RGB (Hex)"</h2>
                        <span>(color)</span>
                    </div>
                    <div>
                        <h2>"RGB (Red, Green, Blue)"</h2>
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
                </div>
                <div class="section">
                    <div>"Web Safe Colors"</div>
                    <div>
                        let vs = ["00", "33", "66", "99", "CC", "FF"];
                        <table>
                            for r in vs.iter() {
                                <tr>
                                    for g in vs.iter() {
                                        for b in vs.iter() {
                                            let v = format!("#{}{}{}", r, g, b);
                                            <td>
                                                <form action="/" method="get">
                                                    <input
                                                        type="hidden"
                                                        name="c"
                                                        value=(&v)
                                                    />
                                                    <button
                                                        style=(format!(
                                                            "background-color: {}; border-width: 0; width: 16px; height: 16px; display: inline-block;",
                                                            v,
                                                        ))
                                                        type="submit"
                                                    ></button>
                                                </form>
                                            </td>
                                        }
                                    }
                                </tr>
                            }
                        </table>
                    </div>
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

fn rgb(value: &str) -> (u8, u8, u8) {
    let value = value.strip_prefix('#').unwrap_or("000000");
    if value.len() != 6 || value.chars().any(|c| !c.is_ascii_hexdigit()) {
        return (0, 0, 0);
    }
    let r = u8::from_str_radix(&value[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&value[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&value[4..6], 16).unwrap_or(0);
    (r, g, b)
}

#[::topcoat::runtime::procedure]
async fn color_to_rgb(value: String, f: f64) -> ::topcoat::Result<f64> {
    let (r, g, b) = rgb(&value);
    if !(f == 0.0 || f == 1.0 || f == 2.0) {
        return Ok(0.0);
    }
    Ok(match f {
        0.0 => r as f64,
        1.0 => g as f64,
        2.0 => b as f64,
        _ => unreachable!(),
    })
}

#[::topcoat::runtime::procedure]
async fn hex_str_to_f64(value: String) -> ::topcoat::Result<f64> {
    Ok(u8::from_str_radix(&value, 16).unwrap_or(0) as f64)
}

#[::topcoat::runtime::procedure]
async fn str_to_f64(value: String) -> ::topcoat::Result<f64> {
    Ok(value.parse::<f64>().unwrap_or(0.0))
}
