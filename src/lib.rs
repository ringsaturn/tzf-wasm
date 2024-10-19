use wasm_bindgen::prelude::*;
use tzf_rs::DefaultFinder;

#[wasm_bindgen]
pub struct WasmFinder {
    finder: DefaultFinder,
}

#[wasm_bindgen]
impl WasmFinder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmFinder {
        WasmFinder {
            finder: DefaultFinder::default(),
        }
    }

    #[wasm_bindgen]
    pub fn get_tz(&self, lng: f64, lat: f64) -> String {
        self.finder.get_tz_name(lng, lat).to_string()
    }

    #[wasm_bindgen]
    pub fn data_version(&self) -> String {
        self.finder.data_version().to_string()
    }
}
