mod game;
use game::Pack;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> String {
    let mut pack = Pack::new();
//    dbg!(&pack);
    pack.shuffle();
    pack.print()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}