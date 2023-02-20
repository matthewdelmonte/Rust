use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     // Convert the Point to a JSON string.
//     let serialized = serde_json::to_string(&point).unwrap();

//     // Prints serialized = {"x":1,"y":2}
//     println!("serialized = {}", serialized);

//     // Convert the JSON string back to a Point.
//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();

//     // Prints deserialized = Point { x: 1, y: 2 }
//     println!("deserialized = {:?}", deserialized);

// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point)?;

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Try to deserialize an invalid JSON string.
    let invalid_json = "{ x: 1, y: 2 }";
    let deserialized: Result<Point, serde_json::Error> = serde_json::from_str(invalid_json);

    // Check if deserialization produced an error.
    if let Err(e) = deserialized {
        eprintln!("Error deserializing point: {}", e);
        return Err(Box::new(e));
    }

    // Prints deserialized = Point { x: 1, y: 2 }
    let deserialized = deserialized.unwrap();
    println!("deserialized = {:?}", deserialized);

    Ok(())
}
