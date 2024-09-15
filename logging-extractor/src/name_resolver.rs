use std::collections::HashMap;

pub fn resolve_name(namespace: &str, aliases: &HashMap<&str, &str>, class: &str) -> String {
    if let Some(stripped) = class.strip_prefix('\\') {
        return stripped.into();
    }
    let (first_part, rest) = class.split_once('\\').unwrap_or((class, ""));
    if let Some(alias) = aliases.get(first_part) {
        if rest.is_empty() {
            alias.to_string()
        } else {
            format!("{alias}\\{rest}")
        }
    } else if namespace.is_empty() {
        class.to_string()
    } else {
        format!("{namespace}\\{class}")
    }
}

#[test]
fn test_resolve_name() {
    use maplit::hashmap;

    assert_eq!(resolve_name("", &hashmap! {}, "Bar"), "Bar");
    assert_eq!(resolve_name("Foo", &hashmap! {}, "Bar"), "Foo\\Bar");
    assert_eq!(
        resolve_name(
            "Foo",
            &hashmap! {
                "Bar" => "Asd"
            },
            "Bar"
        ),
        "Asd"
    );
    assert_eq!(resolve_name("Foo", &hashmap! {}, "\\Bar"), "Bar");
    assert_eq!(resolve_name("Foo", &hashmap! {}, "\\Bar"), "Bar");
    assert_eq!(
        resolve_name(
            "Foo",
            &hashmap! {
                "Bar" => "Asd"
            },
            "\\Bar"
        ),
        "Bar"
    );
}
