use serde::Serialize;

pub fn convert_enum_to_string<T: Serialize>(val: T) -> String {
    serde_json::to_string(&val)
        .expect("could not convert enum to String")
        .replace("\"", "")
}
