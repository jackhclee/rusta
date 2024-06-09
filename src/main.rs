//! This is crate documentation
use std::fmt::{Display, Formatter};
use serde::Serialize;

mod util;

mod tests;

use std::env;

static mut V: Vec<i32> = Vec::new();

static mut books: Vec<Book> = Vec::new();

struct Title {
    ver: i32
}

struct AppConfig {
    title: Title
}

struct Book {
    title: String,
    price: i32,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Book title: {} price: {}", self.title.as_str(), self.price)
    }
}

pub fn ma(some_input: i32) -> i32 {
    return some_input + 1;
}

fn make(i: i32) -> i32 {
    i * 2
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Serialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
#[post("/index")]
async fn index(info: web::Json<Info>) -> Result<impl Responder> {
    //Ok(format!("Welcome {}!", info.username))
    let i = Info {username: info.username.clone()};
    Ok(web::Json(i))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
/**
pub fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
**/
/**
pub fn main() {
    // let mut ct: i64 = 8;
    // let s = String::from("ABCD");
    // println!("Hello, world! {ct}");
    // ct = 9279827985728754;
    // println!("Hello, world! {ct}");
    // lib::prn();
    // util::prn1();
    // let boxa = makeBox(999);
    // println!("{boxa}");
    //
    // println!("{}",add(1,99));
    // println!("{}",multiply(2,99));

    let app = App {
        title: Title { ver: 10 }
    };


    // prn();
    let mut s = String::new();

    println!("{}", s.capacity());

    let book = Book { title: String::from("HK"), price: 1997 };

    println!("Book {}", book);

    for _ in 0..5 {
        s.push_str("hello");
        println!("len: {} capacity: {}", s.len(), s.capacity());
    }

    let i = 99;

    let ri = &i;

    let bi = Box::new(i);

    println!("{}", make(i));

    println!("{}", ri);

    println!("haha");

    let a = [1, 2, 3];

    let mut iter = a.iter();

    for i in a {
        println!("#{i}");
    }

    unsafe {
        V.push(1);
        V.push(4);
        books.push(Book {title: String::from("ABCD"), price: 32});
        println!("{:?}", &V);
    }

    println!("{name}", name = "HK")
}
**/

fn makeBox(num: i32) -> Box<i32> {
    return Box::new(num);
}

mod lib {
    pub fn prn() {
        println!("HHHH");
    }
}
