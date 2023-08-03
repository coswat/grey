#[cfg(test)]
use grey::builder::App;

#[test]
fn test_empty_app() {
    let app: App = App::new();
    assert_eq!(app.name, "");
    assert_eq!(app.desc, "");
    assert_eq!(app.version, "");
}

#[test]
fn test_demo_app() {
    let mut app: App = App::new();
    app.name("test");
    app.version("1.0.0");
    app.description("test app");
    assert_eq!(app.name, "test");
    assert_eq!(app.version, "1.0.0");
    assert_eq!(app.desc, "test app");
}