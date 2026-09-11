use tzf_rs::DefaultFinder;
use wasm_bindgen::prelude::*;

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
    pub fn timezonenames(&self) -> Box<[JsValue]> {
        self.default_finder
            .timezonenames()
            .iter()
            .map(|&name| JsValue::from_str(name))
            .collect::<Vec<JsValue>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen]
    pub fn data_version(&self) -> String {
        self.default_finder.data_version().to_string()
    }

    /// GeoJSON FeatureCollection of the timezone's polygons, or `undefined`
    /// when the dataset does not contain the name.
    #[wasm_bindgen]
    pub fn get_tz_polygon_geojson(&self, tz_name: &str) -> Option<String> {
        self.default_finder
            .get_tz_geojson(tz_name)
            .map(|b| b.to_string())
    }

    /// GeoJSON FeatureCollection of the timezone's preindex tiles, or
    /// `undefined` when no tile names the timezone.
    #[wasm_bindgen]
    pub fn get_tz_index_geojson(&self, tz_name: &str) -> Option<String> {
        self.default_finder
            .get_tz_preindex_geojson(tz_name)
            .map(|b| b.to_string())
    }
}

impl Default for WasmFinder {
    fn default() -> Self {
        Self::new()
    }
}
