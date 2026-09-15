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
                        <h2>"Name"</h2>
                        <span>
                            (NAMED_COLORS
                                .iter()
                                .find(|it| it.1.eq_ignore_ascii_case(&color))
                                .map(|it| it.0)
                                .unwrap_or("(none)"))
                        </span>
                    </div>
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
                                                    <input type="hidden" name="c" value=(&v) />
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

                <div class="section">
                    <div>"Named Colors"</div>
                    <div>
                        <ul>
                            for (name, color) in NAMED_COLORS {
                                <li>
                                    <form action="/" method="get">
                                        <input type="hidden" name="c" value=(&color) />
                                        <button
                                            style=(format!(
                                                "background-color: {}; border-width: 0; width: 16px; height: 16px; display: inline-block;",
                                                color,
                                            ))
                                            type="submit"
                                        ></button>
                                        <span>(name)</span>
                                    </form>
                                </li>
                            }
                        </ul>
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

const NAMED_COLORS: [(&'static str, &'static str); 148] = [
    ("aliceblue", "#f0f8ff"),
    ("antiquewhite", "#faebd7"),
    ("aqua", "#00ffff"),
    ("aquamarine", "#7fffd4"),
    ("azure", "#f0ffff"),
    ("beige", "#f5f5dc"),
    ("bisque", "#ffe4c4"),
    ("black", "#000000"),
    ("blanchedalmond", "#ffebcd"),
    ("blue", "#0000ff"),
    ("blueviolet", "#8a2be2"),
    ("brown", "#a52a2a"),
    ("burlywood", "#deb887"),
    ("cadetblue", "#5f9ea0"),
    ("chartreuse", "#7fff00"),
    ("chocolate", "#d2691e"),
    ("coral", "#ff7f50"),
    ("cornflowerblue", "#6495ed"),
    ("cornsilk", "#fff8dc"),
    ("crimson", "#dc143c"),
    ("cyan", "#00ffff"),
    ("darkblue", "#00008b"),
    ("darkcyan", "#008b8b"),
    ("darkgoldenrod", "#b8860b"),
    ("darkgray", "#a9a9a9"),
    ("darkgreen", "#006400"),
    ("darkgrey", "#a9a9a9"),
    ("darkkhaki", "#bdb76b"),
    ("darkmagenta", "#8b008b"),
    ("darkolivegreen", "#556b2f"),
    ("darkorange", "#ff8c00"),
    ("darkorchid", "#9932cc"),
    ("darkred", "#8b0000"),
    ("darksalmon", "#e9967a"),
    ("darkseagreen", "#8fbc8f"),
    ("darkslateblue", "#483d8b"),
    ("darkslategray", "#2f4f4f"),
    ("darkslategrey", "#2f4f4f"),
    ("darkturquoise", "#00ced1"),
    ("darkviolet", "#9400d3"),
    ("deeppink", "#ff1493"),
    ("deepskyblue", "#00bfff"),
    ("dimgray", "#696969"),
    ("dimgrey", "#696969"),
    ("dodgerblue", "#1e90ff"),
    ("firebrick", "#b22222"),
    ("floralwhite", "#fffaf0"),
    ("forestgreen", "#228b22"),
    ("fuchsia", "#ff00ff"),
    ("gainsboro", "#dcdcdc"),
    ("ghostwhite", "#f8f8ff"),
    ("gold", "#ffd700"),
    ("goldenrod", "#daa520"),
    ("gray", "#808080"),
    ("green", "#008000"),
    ("greenyellow", "#adff2f"),
    ("grey", "#808080"),
    ("honeydew", "#f0fff0"),
    ("hotpink", "#ff69b4"),
    ("indianred", "#cd5c5c"),
    ("indigo", "#4b0082"),
    ("ivory", "#fffff0"),
    ("khaki", "#f0e68c"),
    ("lavender", "#e6e6fa"),
    ("lavenderblush", "#fff0f5"),
    ("lawngreen", "#7cfc00"),
    ("lemonchiffon", "#fffacd"),
    ("lightblue", "#add8e6"),
    ("lightcoral", "#f08080"),
    ("lightcyan", "#e0ffff"),
    ("lightgoldenrodyellow", "#fafad2"),
    ("lightgray", "#d3d3d3"),
    ("lightgreen", "#90ee90"),
    ("lightgrey", "#d3d3d3"),
    ("lightpink", "#ffb6c1"),
    ("lightsalmon", "#ffa07a"),
    ("lightseagreen", "#20b2aa"),
    ("lightskyblue", "#87cefa"),
    ("lightslategray", "#778899"),
    ("lightslategrey", "#778899"),
    ("lightsteelblue", "#b0c4de"),
    ("lightyellow", "#ffffe0"),
    ("lime", "#00ff00"),
    ("limegreen", "#32cd32"),
    ("linen", "#faf0e6"),
    ("magenta", "#ff00ff"),
    ("maroon", "#800000"),
    ("mediumaquamarine", "#66cdaa"),
    ("mediumblue", "#0000cd"),
    ("mediumorchid", "#ba55d3"),
    ("mediumpurple", "#9370db"),
    ("mediumseagreen", "#3cb371"),
    ("mediumslateblue", "#7b68ee"),
    ("mediumspringgreen", "#00fa9a"),
    ("mediumturquoise", "#48d1cc"),
    ("mediumvioletred", "#c71585"),
    ("midnightblue", "#191970"),
    ("mintcream", "#f5fffa"),
    ("mistyrose", "#ffe4e1"),
    ("moccasin", "#ffe4b5"),
    ("navajowhite", "#ffdead"),
    ("navy", "#000080"),
    ("oldlace", "#fdf5e6"),
    ("olive", "#808000"),
    ("olivedrab", "#6b8e23"),
    ("orange", "#ffa500"),
    ("orangered", "#ff4500"),
    ("orchid", "#da70d6"),
    ("palegoldenrod", "#eee8aa"),
    ("palegreen", "#98fb98"),
    ("paleturquoise", "#afeeee"),
    ("palevioletred", "#db7093"),
    ("papayawhip", "#ffefd5"),
    ("peachpuff", "#ffdab9"),
    ("peru", "#cd853f"),
    ("pink", "#ffc0cb"),
    ("plum", "#dda0dd"),
    ("powderblue", "#b0e0e6"),
    ("purple", "#800080"),
    ("rebeccapurple", "#663399"),
    ("red", "#ff0000"),
    ("rosybrown", "#bc8f8f"),
    ("royalblue", "#4169e1"),
    ("saddlebrown", "#8b4513"),
    ("salmon", "#fa8072"),
    ("sandybrown", "#f4a460"),
    ("seagreen", "#2e8b57"),
    ("seashell", "#fff5ee"),
    ("sienna", "#a0522d"),
    ("silver", "#c0c0c0"),
    ("skyblue", "#87ceeb"),
    ("slateblue", "#6a5acd"),
    ("slategray", "#708090"),
    ("slategrey", "#708090"),
    ("snow", "#fffafa"),
    ("springgreen", "#00ff7f"),
    ("steelblue", "#4682b4"),
    ("tan", "#d2b48c"),
    ("teal", "#008080"),
    ("thistle", "#d8bfd8"),
    ("tomato", "#ff6347"),
    ("turquoise", "#40e0d0"),
    ("violet", "#ee82ee"),
    ("wheat", "#f5deb3"),
    ("white", "#ffffff"),
    ("whitesmoke", "#f5f5f5"),
    ("yellow", "#ffff00"),
    ("yellowgreen", "#9acd32"),
];
