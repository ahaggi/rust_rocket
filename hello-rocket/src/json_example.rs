// an `extern crate` loading macros must be at the crate rootrustc(E0468)
// #[macro_use] extern crate rocket_contrib; // this is to use json!({ "status": "ok" })

use serde::{Serialize, Deserialize};
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize, Debug)] //TODO #[derive(.., Copy, Clone)]
pub struct ItemStruct {
    id: u32,
    description: String, //TODO &str
}

/* 
POST http://localhost:8000/item HTTP/1.1
User-Agent: Fiddler
Host: localhost:8000
Content-Type: application/json
Accept: application/json

{"id":2 , "description":"abc" }
 */

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
pub fn items() -> JsonValue { 
    
    // let item = ItemStruct{id: 0 ,description: "mock_data".to_string()};
    // let mut items =  [item;10] ;

    // for i in 1..10  {
    //     items[i] = ItemStruct{id: 0 ,description: "mock_data".to_string()};
    // }

    json!( [
    { "id": 1, "description": "Item1" },
    { "id": 2, "description": "Item2" },
    { "id": 3, "description": "Item3" },
    { "id": 4, "description": "Item4" },
    { "id": 5, "description": "Item5" },
    { "id": 6, "description": "Item6" },
    { "id": 7, "description": "Item7" },
    { "id": 8, "description": "Item8" },
    { "id": 9, "description": "Item9" },
    { "id": 10, "description": "Item10" }])  
 }
