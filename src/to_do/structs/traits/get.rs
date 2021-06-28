use serde_json::Map;
use serde_json::value::Value;

pub trait Get {
    // the &self parameter enables the struct calling the function directly
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match (item) {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            },
            None => println!("Item: {} was not found", title)
        }
    }
}