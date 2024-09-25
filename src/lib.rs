//! [![github]](https://github.com/carloskiki/leptos-icons)&ensp;[![crates-io]](https://crates.io/crates/leptos_icons)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! A simple component that reactively renders an icon.
//!
//! To render icons, this crate needs to be coupled with [`icondata`](https://docs.rs/icondata/latest/icondata/),
//! which is an icon source providing over 20,000 icons.
//!
//! # Getting Started
//!
//! In your Cargo.toml, include both `leptos_icons` and `icondata`:
//!
//! ```toml
//! [dependencies]
//! leptos_icons = { version = "{crate_version}" }
//! icondata = { version = "{icondata_version}" }
//! ```
//!
//! In your leptos project, use:
//! ```
//! use leptos::*;
//! use leptos_icons::*;
//!
//! # #[cfg(target_arch = "wasm32")]
//! let _ = view! {
//!     <Icon icon=icondata::BsFolder />
//! };
//! ```
//! [__Complete examples__](https://github.com/carloskiki/leptos-icons/tree/main/examples) are available on github.

use leptos::html::HtmlElement;
use leptos::prelude::*;
use leptos::svg::*;
use leptos::text_prop::TextProp;

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: MaybeSignal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: MaybeProp<TextProp>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: MaybeProp<TextProp>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: MaybeProp<TextProp>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: MaybeProp<TextProp>,
) -> impl IntoView
where
{
    let svg = move || {
        let icon = icon.get();

        let classes = class.get().map(|c| c.get().to_string());

        let svg: HtmlElement<Svg, (), ()> = leptos::svg::svg();
        let svg = svg
            .inner_html(icon.data)
            .class(classes.unwrap_or("".to_string()))
            .style(match (style.get(), icon.style) {
                (Some(a), Some(b)) => format!("{b} {}", a.get()),
                (Some(a), None) => a.get().to_string(),
                (None, Some(b)) => b.to_string(),
                (None, None) => "".to_string(),
            })
            .attr(
                "width",
                match (width.get(), icon.width) {
                    (Some(a), _) => Oco::from(a.get()),
                    _ => Oco::from("1em"),
                },
            )
            .attr(
                "height",
                match (height.get(), icon.height) {
                    (Some(a), _) => Oco::from(a.get()),
                    _ => Oco::from("1em"),
                },
            )
            .attr("x", icon.x)
            .attr("y", icon.y)
            .attr("height", "1rem")
            .attr("viewBox", icon.view_box)
            .attr("stroke_linecap", icon.stroke_linecap)
            .attr("stroke_linejoin", icon.stroke_linejoin)
            .attr("stroke", icon.stroke)
            .attr("fill", icon.fill.unwrap_or("currentColor"))
            .attr("role", "graphics-symbol");
        svg
    };
    IntoView::into_view(svg)
}
