use std::ops::Add;
use std::fmt;


pub struct MyPoint {
    x: u32,
    y: u32,
}

impl Add for MyPoint {
    type Output = MyPoint;

    fn add(self, other: MyPoint) -> MyPoint {
        MyPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for MyPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyPoint {{ x: {}, y: {} }}", self.x, self.y)
    }
}

pub struct MyImage {
    width: u32,
    height: u32,
    elements: Vec<u8>,
}


impl Add for MyImage {
    type Output = MyImage;

    fn add(self, other: MyImage) -> MyImage {

        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        let elements = self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(a, b)| a.saturating_add(*b)) // prevent overflow
            .collect();

        MyImage {
            width: self.width,
            height: self.height,
            elements,
        }
    }
}
impl fmt::Display for MyImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyImage {{ width: {}, height: {}, elements:\n", self.width, self.height)?;
        for (index, element) in self.elements.iter().enumerate() {
            write!(f, "{:3} ", element)?;
            if (index + 1) % self.width as usize == 0 {
                write!(f, "\n")?;
            }
        }
        write!(f, "}}")
    }
}
pub fn myadd(){
    let p1 = MyPoint { x: 1, y: 2 };
    let p2 = MyPoint { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("p3: {}", p3);

    let i1 = MyImage {
        width: 2,
        height: 3,
        elements: vec![1, 2, 3, 4, 5, 6],
    };
    let i2 = MyImage {
        width: 2,
        height: 3,
        elements: vec![7, 8, 9, 10, 11, 12],
    };
    let i3 = i1 + i2;
    println!("i3: {}", i3);
}