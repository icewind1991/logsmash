use std::collections::HashMap;

pub fn resolve_name(namespace: &str, aliases: &HashMap<&str, &str>, class: &str) -> String {
    if class.starts_with('\\') {
        return class[1..].into();
    }
    let (first_part, rest) = class.split_once('\\').unwrap_or((class, ""));
    if let Some(alias) = aliases.get(first_part) {
        if rest.is_empty() {
            format!("{alias}")
        } else {
            format!("{alias}\\{rest}")
        }
    } else if namespace.is_empty() {
        if rest.is_empty() {
            format!("{first_part}")
        } else {
            format!("{first_part}\\{rest}")
        }
    } else {
        if rest.is_empty() {
            format!("{namespace}\\{first_part}")
        } else {
            format!("{namespace}\\{first_part}\\{rest}")
        }
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
