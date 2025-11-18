use std::io;

// Data types https://doc.rust-lang.org/book/ch03-02-data-types.html
/*
- Rust is statically mean : every var must known type at compile time

* Scalar types: - intergers, floating-point, booleans and characters
     - Intergers: i8 - u8/ i16 - u16/ i32 - u32/ i64 - u64
     - floating-point: default f64
     - booloean: one byte

* Compound types:
     - tuple: grouping together a number of values with a variety of types into one compound type
     - array: let name: [type; number_of_element]
*/

// Function: define any where
const NUMBER_OF_ARR: i8 = 8;
// Control Flow:
/*
     - if Expression: if x > 5
                      {
                        ...
                      }
                      else
                      {
                      ...
                      }
     - return value from loop
*/

fn main() {
    let x: f32 = 5.4;
    println!("{x}");

    let y = true;
    println!("{y}");
    let y_2: bool = false;
    println!("{y_2}");

    let c: char = 'a';
    println!("{c}");
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    let tup: (i32, u64, char) = (-20, 40, 'f');
    println!("{}", tup.0);

    let arr: [f32; 6] = [1.2, 1.3, 1.4, 1.5, 1.6, 1.7];
    println!(
        "Array is {},{},{},{},{},{}",
        arr[0], arr[1], arr[2], arr[3], arr[4], arr[5]
    );
    println!("Sum of arr : {}", check_sum(arr));

    let string_t: [char; 4] = ['a', 'b', 'c', 'd'];
    printf_string(string_t);
}

fn check_sum(arr: [f32; 6]) -> f64 {
    let mut sum: f32 = 0.0;
    let mut i = 0;
    let res: f64 = loop {
        if i < 6 {
            sum = sum + arr[i];
            i = i + 1;
        } else {
            break sum.into();
        }
    };
    return res / 2.0;
}

fn printf_string(string: [char; 4]) {
    for element in string {
        println!("{}", element);
    }
}
