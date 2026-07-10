use project_rust_kvstore::Store;

fn main() {
    let mut store = Store::new();
    store.set("name".to_string(), "kv-store".to_string());

    match store.get("name") {
        Some(value) => println!("Got: {}", value),
        None => println!("Key not found"),
    }
}