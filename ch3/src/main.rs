fn main() {
    // 변수 선언
    // 기본적으로 불변성
    // mut로 변하게 할 수 있음
    let x = 5;
    // 상수 선언
    // 타입이 반드시 명시되어야 한다.
    const Y: u32 = 6;
    // 새도잉
    // 다른 타입으로 재선언
    let x:f64 = 3.14;
    // i32 부호 있음
    // u32 부호 없음
    // f32 부동 소수점
    // bool
    // char 문자
    let c: char = 'a';
    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{y}");
    let x = tup.0;
    println!("{x}");
    // 배열
    let arr = [1,2,3,4,5];
    println!("{}", arr[0]);
    let arr = [3; 5];   // 3으로 채워진 5개 요소
    println!("{}", arr[0]);

    test(123);
    println!("{}", test2());

    // 조건문
    if (arr[0] > 10) {
        println!("10보다 크다");
    } else {
        println!("10보다 작다");
    }

    let number = if arr[0] > 5 { 5 } else { 6 };
    println!("{}", number);

    // 반복문
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("{}", result);

    while count != 0 {
        count -= 1;
    }
    println!("{}", count);

    for a in arr {
        println!("{}", a);
    }
}

fn test(x: i32) {   // 매개변수의 타입을 반드시 선언해야 한다.
    println!("test");
    println!("{}", x);
}

fn test2 () -> i32{
    345 + 23
}


