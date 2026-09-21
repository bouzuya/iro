use crate::Hsl;
use crate::NamedColors;
use crate::Rgb;

#[::topcoat::router::query_params(error = bad_request)]
struct HomeQueryParams {
    c: Option<String>,
}

#[::topcoat::router::page]
async fn home(cx: &::topcoat::context::Cx) -> ::topcoat::Result<impl ::topcoat::view::View> {
    let HomeQueryParams { c } = ::topcoat::router::query_params::<HomeQueryParams>(cx)?;
    let c = c
        .as_deref()
        .and_then(|s| Rgb::from_hex(s).ok())
        .unwrap_or_else(Rgb::bouzuya_green);

    let red = ::topcoat::runtime::signal(cx, || c.r() as f64);
    let green = ::topcoat::runtime::signal(cx, || c.g() as f64);
    let blue = ::topcoat::runtime::signal(cx, || c.b() as f64);

    let rgb = Rgb::new(red.get() as u8, green.get() as u8, blue.get() as u8);
    let color = rgb.to_hex();

    Ok(::topcoat::view::view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>"iro"</title>
                <link href=(::topcoat::asset::asset!("../../index.css")) rel="stylesheet" />
                ::topcoat::dev::script()
                ::topcoat::runtime::script()
            </head>
            <body>
                <div class="section">
                    <div>
                        <h2>"Color Picker"</h2>
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
                    </div>
                    <div>
                        <h2>"Name"</h2>
                        <span>
                            (NamedColors::find_name_by_hex(&color)
                                .unwrap_or("(none)")
                                .to_string())
                        </span>
                    </div>
                    <div>
                        <h2>"RGB (Hex)"</h2>
                        <label>
                            <span>"Hex"</span>
                            <form action="/" method="get">
                                <input name="c" type="text" :value=$(color) />
                                <button type="submit">"Submit"</button>
                            </form>
                        </label>
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
                    <div>
                        <h2>"HSL (Hue, Saturation, Lightness)"</h2>
                        let hsl = Hsl::from_rgb(rgb);
                        <div>
                            (format!("hsl({}deg, {}%, {}%)", hsl.h(), hsl.s(), hsl.l()))
                        </div>
                    </div>
                </div>
                <div class="section">
                    <div>"Web Safe Colors"</div>
                    <div>
                        let vs = ["00", "33", "66", "99", "CC", "FF"];
                        <table>
                            for r in vs.iter() {
                                for g in vs.iter() {
                                    <tr>
                                        for b in vs.iter() {
                                            let v = format!("#{}{}{}", r, g, b);
                                            <td>color_chip_form(color: &v)</td>
                                        }
                                    </tr>
                                }
                            }
                        </table>
                    </div>
                </div>

                <div class="section">
                    <div>"Named Colors"</div>
                    <div>
                        <ul>
                            for (name, color) in NamedColors {
                                <li>
                                    <div class="named_color_item">
                                        color_chip_form(color: &color)
                                        <span>(name)</span>
                                    </div>
                                </li>
                            }
                        </ul>
                    </div>
                </div>
            </body>
        </html>
    })
}

#[::topcoat::view::component]
async fn color_chip_form(color: &str) -> ::topcoat::Result<impl ::topcoat::view::View> {
    Ok(::topcoat::view::view! {
        <form action="/" method="get">
            <input type="hidden" name="c" value=(&color) />
            <button
                class="color_chip_button"
                style=(format!("background-color: {}", color))
                type="submit"
            ></button>
        </form>
    })
}

#[::topcoat::runtime::procedure]
async fn color_to_rgb(value: String, f: f64) -> ::topcoat::Result<f64> {
    let rgb = Rgb::from_hex(&value).unwrap_or_else(|_| Rgb::new(0, 0, 0));
    let (r, g, b) = (rgb.r(), rgb.g(), rgb.b());

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
