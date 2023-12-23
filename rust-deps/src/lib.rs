use wasm_bindgen::prelude::*;
use js_sys::*;

#[wasm_bindgen]
pub fn deep_throat(input_value: JsValue) -> JsValue {
   deep_copy(input_value) 
}

#[wasm_bindgen]
pub fn deep_copy(js_value: JsValue) -> JsValue {
    match js_value.clone().as_string() {
        Some(s) => {
            JsValue::from_str(&s)
        }
        None => {
            if js_value.is_object() {
                let object = js_value.dyn_into::<Object>().unwrap();
                let result = Object::new();

                let keys = Reflect::own_keys(&object).unwrap();
                for i in 0..keys.length() {
                    let key = keys.get(i);
                    let value = js_sys::Reflect::get(&object, &key).unwrap();

                    let copied_key = deep_copy(JsValue::from(key));
                    let copied_value = deep_copy(JsValue::from(&value));

                    Reflect::set(&result, &copied_key, &copied_value);
                }

                result.into()
            } else {
                js_value.clone()
            }
        }
    }
}
