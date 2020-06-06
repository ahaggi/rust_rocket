use rocket::response::content;
use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct HitCount {
    pub count: AtomicUsize,
}
// TODO Try to use this instead of using a struct "HitCount" as a managed state
// pub static request_id_counter: AtomicUsize = AtomicUsize::new(0);

#[get("/ms/increase_count")]
pub fn increase_count(hit_count: State<HitCount>) -> content::Html<String> {
    hit_count.count.fetch_add(1, Ordering::Relaxed);
    let msg1 = "Your visit has been recorded!";
    let msg2 = format!("Visits: {}", count(hit_count));
    content::Html(format!("{}<br /><br />{}", msg1, msg2))
}

#[get("/ms/count")]
pub fn count(hit_count: State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}


/* 
TODO debug


impl<'a, 'r> FromRequest<'a, 'r> for ??????? {
    fn from_request(req: &'a Request<'r>) -> String {      //rocket::Request
    let hit_count_state = req.guard::<State<HitCount>>()?;
    let current_count = hit_count_state.count.load(Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}
*/