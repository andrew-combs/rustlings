// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref mut p) => p.x = 2000,
        _ => println!("no match"),
    }
    println!("{:?}", y);
    y; // Fix without deleting this line.
}
