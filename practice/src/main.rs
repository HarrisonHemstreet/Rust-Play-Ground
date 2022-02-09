fn main() {
    let x = what_centry_is_it(1749);
    println!("the answer is: {}", x);

    let is_pal = is_palindrome("pool".to_string());
    println!("is pal: {}", is_pal);

    let mut new_vec = vec![1, 1, -1, 10, -10, 1];
    println!("wowza {:?}", adjacent_elements_product(new_vec));
}

// return the century when given a year. For example, 1749 will return 18
// as in the 18th centry
fn what_centry_is_it(x: i32) -> i32 {
    let mut w: f32 = x as f32;
    w = w / 100 as f32;
    let y: i32 = w.ceil() as i32;
    let z: i32 = w.floor() as i32;
    if y > z {
       y
    } else {
       z
    }
}

fn is_palindrome(inputString: String) -> bool {
    let rev_string: String = inputString.chars().rev().collect();

    inputString == rev_string
}

// Given an array of integers, find the pair of adjacent elements
// that has the largest product and return that product.
fn adjacent_elements_product(inputArray: Vec<i32>) -> i32 {
    let mut largest_product: i32 = inputArray[0] * inputArray[1];
    let mut prev_num: i32 = inputArray[0];
    for i in inputArray {
        if i == largest_product {
            largest_product = i;
        }
        if i == prev_num {
            continue;
        }
        if (i * prev_num) > largest_product {
            largest_product = i * prev_num;
            prev_num = i;
        }
        else {
            prev_num = i;
        }
    }
    return largest_product;
}

