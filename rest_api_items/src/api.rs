use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize, Debug)] //TODO #[derive(.., Copy, Clone)]
pub struct ItemStruct {
    id: u32,
    description: String, //TODO &str
}

#[post("/item", format = "json", data = "<item>")]
pub fn item_post(item: Json<ItemStruct>) -> JsonValue { 
    println!("rcvd {:#?}", item.0);
    println!("description is {}", item.0.description);
    println!("id is {}", item.0.id);
    json!({ "status": "ok" })

 }

#[get("/items/<id>")]
pub fn get_item(id: u32) -> Json<ItemStruct> { 

    let item: ItemStruct = ItemStruct{id:id ,description: "mock_data".to_string()};
    println!("resp with {:#?}", item);

    Json(item)  //rocket_contrib::json::Json

 }

#[get("/items")]
pub fn items() -> String { 
    
    let mut values: Vec<ItemStruct> = (1..10).map(|x| ItemStruct{id: x ,description: "mock_data".to_string()}).collect();
    let res = serde_json::to_string(&values);

    match res {
        Ok(items) => items,
        _ => "[]".to_string() // TODO implement a better ErrHandling mechanism!!
    }

    // json!( [
    // { "id": 1, "description": "Item1" },
    // { "id": 2, "description": "Item2" },
    // { "id": 3, "description": "Item3" },
    // { "id": 4, "description": "Item4" },
    // { "id": 5, "description": "Item5" },
    // { "id": 6, "description": "Item6" },
    // { "id": 7, "description": "Item7" },
    // { "id": 8, "description": "Item8" },
    // { "id": 9, "description": "Item9" },
    // { "id": 10, "description": "Item10" }])  
 }
