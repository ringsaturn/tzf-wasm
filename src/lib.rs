use lazy_static::lazy_static;
use tzf_rs::DefaultFinder;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::default();
}

#[wasm_bindgen(js_name = getTz)]
pub fn get_tz_name(lng: f64, lat: f64) -> JsValue {
    let tz = FINDER.get_tz_name(lng, lat);
    JsValue::from_str(&tz)
}
