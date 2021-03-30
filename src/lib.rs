use wasm_bindgen::prelude::*;
use owl_fts::FTS;
use js_sys::{Array, Reflect, Object};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct FTSInstance {
    fts: FTS
}

#[wasm_bindgen]
pub fn setup(input: &str) -> FTSInstance {
    return FTSInstance {
        fts: FTS::new(input).unwrap()
    }
}

#[wasm_bindgen]
pub fn search(instance: &FTSInstance, term: &str) -> Array {
    let arr = Array::new();

    instance.fts
        .search(term)
        .into_iter()
        .for_each(|r| {
            let obj = Object::new();
            Reflect::set(&obj, &"anchor".into(), &r.page_id.into()).unwrap();
            Reflect::set(&obj, &"score".into(), &r.score.into()).unwrap();
            arr.push(&obj);
        });

    arr
}
