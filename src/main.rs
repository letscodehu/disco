#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use std::borrow::Borrow;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::databases::mysql::Conn;

use crate::core::entity::Post;
use crate::core::repository::get_post_by_id;
use rocket_contrib::templates::Template;

pub mod core {
    pub mod entity {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Post {
            pub id: i32,
            pub name: String,
        }

    }

    pub mod repository {
        use rocket_contrib::databases::mysql::Conn;
        use crate::core::entity::Post;

        pub fn get_post_by_id(conn: &mut Conn, id: &str) -> Post {
            let mut statement = conn.prepare("SELECT * FROM posts WHERE id = ?").unwrap();
            statement.execute(vec![id]).unwrap().last()
                .unwrap().map(|row| {
                Post {
                    id : row.get(0).unwrap(),
                    name : row.get(1).unwrap(),
                }
            }).unwrap()
        }
    }
}


fn main() {

    #[get("/post/<id>")]
    fn post(mut conn: MySQLDbConnection, id: String) -> String {
        serde_json::to_string(get_post_by_id(&mut *conn, id.as_str()).borrow()).unwrap()
    }

    #[get("/")]
    fn index() -> Template {
        Template::render("index", {})
    }

    #[database("mysql")]
    struct MySQLDbConnection(Conn);

    rocket::ignite()
        .attach(Template::fairing())
        .attach(MySQLDbConnection::fairing())
        .mount("/", StaticFiles::from("dist"))
        .mount("/", routes![post, index]).launch();
}