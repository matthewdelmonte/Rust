// New Struct for rectangle dimensions
struct Dimensions {
    width: u32,
    height: u32,

    getArea: fn area(width: u32, height: u32) -> u32 {
    width * height
}

}

fn main() {
    // instantiating an instance of Dimentions
    let rec = Dimensions {
        width: 30,
        height: 50,
    };

    let width = rec.width;
    let height = rec.height;

    println!("The area of the rectangle is {} square pixels", area(width, height));
}

// function that returns the area
fn area(width: u32, height: u32) -> u32 {
    width * height
}
