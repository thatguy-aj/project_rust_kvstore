use project_rust_kvstore::Store;

fn main() -> std::io::Result<()> {
    let mut store = Store::load("data.log")?;
    store.set("name".to_string(), "kv-store".to_string())?;

    if let Some(value) = store.get("name") {
        println!("Got: {}", value);
    }

    Ok(())
}
