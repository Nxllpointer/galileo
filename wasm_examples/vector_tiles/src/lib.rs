use galileo::MapBuilder;
use galileo::layer::vector_tile_layer::style::VectorTileStyle;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[path = "../../common.rs"]
mod common;

#[path = "../../../galileo/examples/vector_tiles.rs"]
mod example;

#[wasm_bindgen]
extern "C" {
    fn send_feature(layer: String, feature_type: String, feature: String);
}

#[wasm_bindgen]
pub fn set_style(style_json: JsValue) {
    // let str = style_json.as_string().unwrap();
    // let style = serde_json::from_str(&str).unwrap_or_else(|_| get_layer_style());
    // let layer = example::LAYER.get().unwrap();
    // layer.write().unwrap().update_style(style);
}

fn get_layer_style() -> VectorTileStyle {
    serde_json::from_str(include_str!("../../../galileo/examples/data/vt_style.json")).unwrap()
}

#[wasm_bindgen]
pub async fn init() {
    // Get your free MapTiler key at https://maptiler.com
    let api_key = std::env!("VT_API_KEY");

    let (container, size) = common::set_up().await;
    example::run(
        MapBuilder::new()
            .with_size(size.width(), size.height())
            .with_container(container),
        get_layer_style(),
        api_key.to_string(),
    )
    .await;
}
