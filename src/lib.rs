use wasm_bindgen::prelude::*;
use tzf_rs::DefaultFinder;
use serde_json::to_string_pretty;

#[wasm_bindgen]
pub struct WasmFinder {
    default_finder: DefaultFinder,
}

#[wasm_bindgen]
impl WasmFinder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmFinder {
        WasmFinder {
            default_finder: DefaultFinder::default(),
        }
    }

    #[wasm_bindgen]
    pub fn get_tz_name(&self, lng: f64, lat: f64) -> String {
        self.default_finder.get_tz_name(lng, lat).to_string()
    }

    #[wasm_bindgen]
    pub fn get_tz_names(&self, lng: f64, lat: f64) -> Box<[JsValue]> {
        self.default_finder
            .get_tz_names(lng, lat)
            .iter()
            .map(|&name| JsValue::from_str(name))
            .collect::<Vec<JsValue>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen]
    pub fn data_version(&self) -> String {
        self.default_finder.data_version().to_string()
    }

    #[wasm_bindgen]
    pub fn get_tz_geojson_from_polygonfinder(&self, tz_name: &str) -> String {
        let boundary_file = self.default_finder.finder.get_tz_geojson(tz_name);
        to_string_pretty(&boundary_file).expect("Failed to serialize GeoJSON")
    }

    #[wasm_bindgen]
    pub fn get_tz_geojson_from_fuzzy(&self, tz_name: &str) -> String {
        let boundary_file = self.default_finder.fuzzy_finder.get_tz_geojson(tz_name);
        to_string_pretty(&boundary_file).expect("Failed to serialize GeoJSON")
    }
}
