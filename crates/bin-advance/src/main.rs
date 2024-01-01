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
// fn dissplay_coordinate(coordinate: (i32, i32)){
//     let (x,y) = coordinate;
//     if y >5{
//         println!("Greater than 5");
//     } else if y < 5 {
//         println!("less than 5");
//     }
//     else if y == 5 {
//         println!("equaln 5");
//     }
// }


// fn main(){
//     let (x,y) = (5,6);
//     let coor = (5,6);
//     let coordinate = (1,17);
//     dissplay_coordinate(coordinate);
//     }

// fn check_value(big: bool){
//     match big {
//         true => println!("it's big"),
//         false => println!("it's small"),

//     }
// }


// fn main(){
//     let my_num = 11;
//     let big = my_num < 100;
//     check_value(big);
// }


//ownership

// enum Light{
//     Bright,
//     Dull
// }
// fn display_light(light: &Light){
//     match light {
//         Light::Bright => println!("Bringt"),
//         Light::Dull => println!("Dull"),

//     }

// }
// fn main(){
//     let dull = Light::Dull;
//     display_light(&dull);
//     display_light(&dull);
// }
// struct Grocery_item{
//     id: i32,
//     quantity: i32,

// }

// fn display_id(grocery: &Grocery_item){
//     println!("{:?}", grocery.id)
// }

// fn display_quantity(grocery: &Grocery_item){
//     println!("{:?}", grocery.quantity)
// }

// fn main(){
//     let grocery = Grocery_item { id: {5}, quantity: {9} };
//     display_id(&grocery);
//     display_quantity(&grocery);

// }


//impl in rust 

// struct Temperature {
//     degree: f64,
// }



// impl Temperature {
//     fn display_temp(&self){
//         println!("{:?}", self.degree);
//     }
// }

// fn main(){
//     let hot = Temperature{degree: 99.9};
//     hot.display_temp();

// }

// struct Dimensions {
//     width: f64,
//     height: f64,
//     depth: f64,
// }

// enum Color {
//     Red,
//     Blue,
//     White,
// }

// struct Box {
//     dimensions: Dimensions,
//     weight: f64,
//     color: Color,

// }

// impl Color {
//     fn print(&self){
//         match self{
//             Color::Blue => println!("Blue"),
//             Color::Red => println!("Red"),
//             Color::White => println!("White"),

//         }
//     }
// }

// impl Dimensions {
//     fn new(width: f64, height: f64, depth: f64) ->Self {
//         Self { width, height, depth }
//     }

//     fn print(&self){
//         println!("Depth: {:?}, Height: {:?}, Width: {:?}", self.depth, self.height, self.width)
//     }
// }

// impl Box {
//     fn new(dimensions: Dimensions, weight: f64, color: Color) ->Self {
//         Self { dimensions, weight, color }
//     }

//     fn print(&self){
//         self.color.print();
//         self.dimensions.print();
//         println!("{:?}", self.weight);
//     }
// }

// fn main(){

//     let dimensions = Dimensions::new(12.3,32.2,33.2);
//     let small_box = Box::new( dimensions,11.1, Color::Blue);
//     small_box.print();
// }



//vector
// fn main(){
    // let my_numbers = vec![1,2,3];
    // for num in my_numbers{
    //     println!("{:?}",num);
    // }


    // let mut my_numbers = Vec::new();    

    // my_numbers.push(1);
    // println!("{:?}", my_numbers);
    // my_numbers.push(2);
    // println!("{:?}", my_numbers);

    // my_numbers.push(3);
    // println!("{:?}", my_numbers);

    // my_numbers.pop();

    // println!("{:?}", my_numbers);

    // my_numbers.len();
    // println!("{:?}", my_numbers.len());

        // let numbers = vec![10,20,30,40];
        // for num in &numbers {
        //     match num {
        //         30 => {println!("Thisty");}
        //         _ =>println!("{:?}", num)
        //     }

        // }


// }



//string in rust 

//owner's string
//borrowed string

// fn print_it(data: &str){
//     println!("{:?}",data);
// }

// struct Employee {
//     name: String,

// }

// fn main(){
//     // print_it("data");
//     // let owned_string = "owned string".to_owned();
//     // let another_owned = String::from("another");
//     // print_it(&owned_string);
//     // print_it(&another_owned);
//     let emp = Employee {
//         // name:"Jayston".to_owned(), //1
//         name: String::from("Jayson"),

//     };
//     println!("{}",emp.name);
// }

// enum Colors {
//     Red,
//     Blue,
//     White,
// }

// impl Colors {
//     fn print(&self){
//         match self {
//             Colors::Red => println!("red"),
//             Colors::Blue => println!("Blue"),
//             Colors::White => println!("White"),

//         }
//     }
// }

// struct Persons {
//     age: i32,
//     name: String,
//     fa_color: Colors,
// }

// impl Persons {
//     fn print(&self){
//         println!("{:?},{:?}",self.age,self.name);
//         self.fa_color.print();
//     }
// }


// fn main(){

//     let person_init = vec![
//         Persons {
//             name: "Pich".to_owned(),
//             age: 9,
//             fa_color: Colors::Red,
//         },

//         Persons {
//             name: "Hoang".to_owned(),
//             age: 12,
//             fa_color: Colors::Blue,
//         },
        
//         Persons {
//             name: "Tran".to_owned(),
//             age: 4,
//             fa_color: Colors::White,
//         },
//     ];

//     for person in person_init {
//         if person.age <= 10 {
//             person.print();
//         }
//     }


// }


//enum advange
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main(){
    let tickets = vec![
        Ticket::Backstage(12.2,"Te".to_owned()),
        Ticket::Vip(15.2,"Tran".to_owned()),
        Ticket::Standard(199.9),

    ];

    //creating for a loop to print out ticket infor

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price,owner ) => {println!("Backstage: {:?}, Owner: {:?}",price, owner)

        },
        Ticket::Vip(price,owner ) => {println!("Vip: {:?}, Owner: {:?}",price, owner)
            
        },
        Ticket::Standard(price ) => {println!("Backstage: {:?}",price)
            
        }
        }
    }
}

// impl Colors {
//     fn print(&self){
//         match self {
//             Colors::Red => println!("red"),
//             Colors::Blue => println!("Blue"),
//             Colors::White => println!("White"),

//         }
//     }
// }

// struct Persons {
//     age: i32,
//     name: String,
//     fa_color: Colors,
// }

// impl Persons {
//     fn print(&self){
//         println!("{:?},{:?}",self.age,self.name);
//         self.fa_color.print();
//     }
// }


// fn main(){

//     let person_init = vec![
//         Persons {
//             name: "Pich".to_owned(),
//             age: 9,
//             fa_color: Colors::Red,
//         },

//         Persons {
//             name: "Hoang".to_owned(),
//             age: 12,
//             fa_color: Colors::Blue,
//         },
        
//         Persons {
//             name: "Tran".to_owned(),
//             age: 4,
//             fa_color: Colors::White,
//         },
//     ];

//     for person in person_init {
//         if person.age <= 10 {
//             person.print();
//         }
//     }


// }
