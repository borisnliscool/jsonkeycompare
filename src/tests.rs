#[cfg(test)]
mod test {
    use serde_json::{from_str, Value};

    use crate::compare_json_keys;

    #[test]
    fn flat_correct() {
        let json1: Value = from_str("{\"a\":\"\"}").unwrap();
        let json2: Value = from_str("{\"a\":\"\"}").unwrap();
        assert!(compare_json_keys(&json1, &json2).is_empty());
    }

    #[test]
    fn nested_correct() {
        let json1: Value = from_str("{\"a\":{\"b\":\"\"}}").unwrap();
        let json2: Value = from_str("{\"a\":{\"b\":\"\"}}").unwrap();
        assert!(compare_json_keys(&json1, &json2).is_empty());
    }

    #[test]
    fn flat_incorrect() {
        let json1: Value = from_str("{\"a\":\"\"}").unwrap();
        let json2: Value = from_str("{\"b\":\"\"}").unwrap();
        assert!(!compare_json_keys(&json1, &json2).is_empty());
    }

    #[test]
    fn nested_incorrect() {
        let json1: Value = from_str("{\"a\":{\"b\":\"\"}}").unwrap();
        let json2: Value = from_str("{\"a\":{\"c\":\"\"}}").unwrap();
        assert!(!compare_json_keys(&json1, &json2).is_empty());
    }

    #[test]
    fn nested_incorrect_2() {
        let json1: Value = from_str("{\"a\":{\"b\":\"\"}}").unwrap();
        let json2: Value = from_str("{\"b\":{\"b\":\"\"}}").unwrap();
        assert!(!compare_json_keys(&json1, &json2).is_empty());
    }

    #[test]
    fn array_correct() {
        let json1: Value = from_str("{\"a\":[\"b\"]}").unwrap();
        let json2: Value = from_str("{\"a\":[\"b\"]}").unwrap();
        assert!(compare_json_keys(&json1, &json2).is_empty());
    }

    #[test]
    fn array_incorrect() {
        let json1: Value = from_str("{\"a\":[\"b\",\"c\"]}").unwrap();
        let json2: Value = from_str("{\"a\":[\"b\"]}").unwrap();
        assert!(!compare_json_keys(&json1, &json2).is_empty());
    }
}