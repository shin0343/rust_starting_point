fn main() { // &
    let my_number = 15; // This is an i32
    let single_reference = &my_number; // refrence to my_number
    let double_reference = &single_reference; // This is a &&i32
    let five_reference = &&&&&my_number; // This is a &&&&&i32
}