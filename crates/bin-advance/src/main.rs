// // #![allow(unused)]
// // use std::net::SocketAddr;

// // use axum::Router;
// // use axum::routing::get;
// // use axum::response::Html;
// // #[tokio::main]
// // async fn main() {
// //     let routes_hello = Router::new().route(
// //         "/hello",
// //         get(|| async{ Html("HelloWorld") }),
// //     );
// //     let addr = SocketAddr::from(([127,0,0,1], 8080));
// //     axum::Server::bind(&addr)
// //     .serve(routes_hello.into_make_service())
// //     .await
// //     .unwrap();
// // }
// // fn main(){
// //     let bool = true;
// //     if bool{
// //         println!("Wrong");
// //     } else {
// //         println!("bye");
// //     }
// // }

// fn main(){
//     // let my_bool = false;
//     // match my_bool {
//     //     true => print!("It's true"),
//     //     _=>println!("It's false"),
//     // }
//     //loop
//     // let mut i = 1;
//     // loop {
//     //     println!("i?{}",i);
//     //     if i == 1 {
//     //         break;
//     //     }
//     //     i = i +1;
//     // }

//     //while
//     let mut i =21;
//     while i>=1{
//             println!("?:{}",i);
//             i = i - 1;

//     }
//     println!("done"); 
// //     //output 
// //     ?:21
// // ?:20
// // ?:19
// // ?:18
// // ?:17
// // ?:16
// // ?:15
// // ?:14
// // ?:13
// // ?:12
// // ?:11
// // ?:10
// // ?:9
// // ?:8
// // ?:7
// // ?:6
// // ?:5
// // ?:4
// // ?:3
// // ?:2
// // ?:1
// // done
// }


// //how enum work?
// enum Color {
//     Red,
//     White,
//     Black,
// }

// fn display_color(color: Color){
//     match color {
//         Color::Black => println!("black"),
//         Color::Red => println!("Red"),
//         Color::White => println!("White"),


//     }
// }

// fn main(){

//     let red = Color::Red;
//     let white = Color::White;
//     let black = Color::Black;

//     display_color(red);
//     display_color(white);
//     display_color(black);

// }


//struct
// enum Flavor {
//     Banana,
//     Banana2,
//     Banana3,
// }
// struct Drink {
//     flavor: Flavor,
//     ounce: f64,
// }

// fn display_drink(drink: Drink){
//     match drink.flavor {
//         Flavor::Banana => println!("it's banana"),
//         Flavor::Banana2 => println!("it's banana2"),
//         Flavor::Banana3 => println!("it's banana3")

//     }
//     println!("{:?}", drink.ounce);
// }

// fn main(){

//     let drink = Drink {
//         flavor: Flavor::Banana,
//         ounce: 1.2,
//     };
//     display_drink(drink);


// }



//tuple



fn main(){
    let (x,y) = (5,6);
    let coor = (5,6);
}