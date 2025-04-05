fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of are: {} {} {}", x, y, z);
    
    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;
    println!("The values of are: {} {} {}", five_hundred, six_point_four, one);

    let _a = [1, 2, 3, 4, 5];
    let first = _a[0];
    let second = _a[1];
    println!("The values of are: {} {}", first, second);
    
    let _months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];

    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    let _c = [3; 5]; // [3, 3, 3, 3, 3]
}
