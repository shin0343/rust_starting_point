fn give_number(one: i16, two: i16) -> i16 {
    let multiplied_by_ten = {
        let first_number = 10;
        // 세미콜론 안붙이면 return first_number * one * two;과 동일하게 동작
        first_number * one * two
    };

    multiplied_by_ten
}

fn main() {
    let my_number = give_number(1, 9);
    println!("{}", my_number);
}