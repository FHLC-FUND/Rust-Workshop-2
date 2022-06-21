#[warn(non_snake_case)]
pub fn main() {
    main1();
    // main2();
    main3();
    main4();
}

// Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
fn main1() {
    let input = 10;
    let x = change_value(input);
    println!("Đáp án bài 1 là: {}", x);
}
fn change_value(input:u32) -> u32{
    if input == 1 {
        let output:u32 = 3;
        return output;
    }
    else {
        let output:u32 = 4;
        return output;
    }
}

//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
// fn main2() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);
//     while count < 5 {
//         num += 2;
//         if vector_is_prime(num, &primes) {
//             count += 1;
//             primes.push(num);
//         }
//     }
// }
// fn vector_is_prime(num: u64, p: &[u64]) -> bool {
//     for &i in p {
//         if num > i && num % i != 0 {
//             return false;
//         }
//     }
//     true
// }

//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
fn main3() {
    let mut values = vec![10, 11, 12];
    let mut max = 0;
    for n in &mut values {
        max = std::cmp::max(max, *n);
    }
    for n in &mut values {
        *n = 100 * (*n) / max;
    }
    println!("Đáp án bài 3 là: {:#?}", values);
}

//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main4(){
    loop {
        let (a, c) = test(vec![1,2,3,4,5]);
        println!("Đáp án bài 4 là: {:?}",a);
        break;
    }
}
pub fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}
