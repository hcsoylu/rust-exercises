// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

struct Person {
    name : String,
    fav_color : String,
    age : i32
}

fn print(data : &str) {
    println!("{:?}", data)
}


fn main() {
     let people = vec![
         Person {
             name : String::from("George"),
             fav_color : String::from("green"),
             age : 7,
         },
         Person {
            name : String::from("Anna"),
            fav_color : String::from("purple"),
            age : 9,
        },
        Person {
            name : String::from("Katie"),
            fav_color : String::from("blue"),
            age : 14,
        },    
     ];

     for person in people {
         if person.age <= 10 {
              print(&person.name);
              print(&person.fav_color)
         }
     }
}
