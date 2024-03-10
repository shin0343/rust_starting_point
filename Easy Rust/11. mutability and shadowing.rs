// mutability
// Rust is immutable by default
// 항상 immutable, 즉 변수 값을 바꿀 수 없음
// 값을 바꾸고 싶으면 데이터 타입 뒤에 mut 붙여야함!

// shadowing : 같은 이름을 다시 쓰는 것
// shadowing한 변수가 사라지면 원래있던 변수를 다시 볼 수 있게됨!

fn main() {
    let mut my_number = 10;
    my_number = 9;
    let my_number = "My Number"; // shadowing
    let my_number = 5; // shadowing

    println!("{}", my_number);
}