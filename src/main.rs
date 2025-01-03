mod my_module;

use my_module::from_file_1;
use my_module::from_file_2;

fn main() {
    from_file_1();
    from_file_2();

    println!("Hello, world!");
}
