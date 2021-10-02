pub fn print_triangle(base_length: i32, height_length: i32) {
    let mut offset = base_length-1;

    for _i in 0..height_length {
        for _j in offset..base_length {
            print!("o");
        }
        offset = offset - 1;
        println!();
    }
}