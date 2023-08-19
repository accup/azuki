fn main() {
    let mut buffer: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    buffer.copy_within(3.., 0);
    println!("{:?}", buffer);

    let mut buffer: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    buffer.copy_within(..5, 3);
    println!("{:?}", buffer);
}
