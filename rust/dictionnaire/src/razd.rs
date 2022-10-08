use dictionnaire::*;
use std::{collections::HashMap, hash::Hash};

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

fn main() {
    let sa = HorVa!( 0 =>"one", 1 => "two" );
    let saa = HorVa!(0 => { 2 => "one" },1 => { 3 => "two" });
    let saa = HorVa!("0" => { "2" => 11 },"1" => { "3" => 12 });
    // let zae = HorVa! {
    //     "left_eye"=> {
    //         "zoom"=> 0,//self.controller.zoom_pos["left"][default_zoom_level]["zoom"],
    //         "focus"=> 0,//self.controller.zoom_pos["left"][default_zoom_level]["focus"],
    //         "speed"=> 0,//default_zoom_speed,
    //         // "zoom_level"=> "default_zoom_level",
    //     },
    //     "right_eye"=> {
    //         "zoom"=> 0,//self.controller.zoom_pos["left"][default_zoom_level]["zoom"],
    //         "focus"=> 0,//self.controller.zoom_pos["left"][default_zoom_level]["focus"],
    //         "speed"=> 0,//default_zoom_speed,
    //         // "zoom_level"=> "default_zoom_level",
    //     },
    // };
    let zae = HorVa! {
        "left_eye" => {
            "zoom" => 0,
            "focus" => 0,
            "speed" => 0,
            "zoom_level" => "default_zoom_level",
        },
        "right_eye" => {
            "zoom" => 0,
            "focus" => 0,
            "speed" => 0,
            "zoom_level" => "default_zoom_level",
        }
    };
    println!("{:?} {:?}", sa, saa);
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }

    println!("{:?}", contacts);
}

// {
//     'left_eye': {
//         'zoom': self.controller.zoom_pos["left"][default_zoom_level]['zoom'],
//         'focus': self.controller.zoom_pos["left"][default_zoom_level]['focus'],
//         'speed': default_zoom_speed,
//         'zoom_level': default_zoom_level,
//     },
//     'right_eye': {
//         'zoom': self.controller.zoom_pos["left"][default_zoom_level]['zoom'],
//         'focus': self.controller.zoom_pos["left"][default_zoom_level]['focus'],
//         'speed': default_zoom_speed,
//         'zoom_level': default_zoom_level,
//     },
// }
