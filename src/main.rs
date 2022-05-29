/*
fn main() {
    /*
    let _x = what_centry_is_it(1749);
    // println!("the answer is: {}", x);

    let _is_pal = is_palindrome("pool".to_string());
    // println!("is pal: {}", is_pal);

    let _new_vec = vec![1, 1, -1, 10, -10, 1];
    // println!("wowza {:?}", adjacent_elements_product(new_vec));
    */

    let this_vec = vec![40, 50, 60, 10, 20, 30];
    /*
    //                  0   1   2   3   4   5
    let this_vec = vec![40, 50, 60, 20, 30];
    //                  0   1   2   3   4

    let this_vec2 = vec![4, 5, 6, 7, 8, 9];

    let this_vec3 = vec![100, 1, 2, 3, 4, 6];
    let this_vec3 = vec![0, 100, 1, 2, 1, 4, 6];
    // if first iteration
    // 100 < 1 < 2
    //
    // and after:
    // 2 < 1 < 4
    // 
    let this_vec3 = vec![100, 1, 200, 300, 400];
    let this_vec4 = vec![1, 50, 60, 60, 70, 80];
    // println!("check this ish {:?}", sort_statues(this_vec));
    */
    almost_increasing_sequence(this_vec);

    /*
     * 1. prevNum, currNum, nextNum
     * 1.1. check if the sequence is greater than 3 number
     * 2. if prevNum < currNum < nextNum, we should be good
    */

    /*
     * 0. create a counter variable to keep track of statues removed
     * 1. loop through sequence
     * 2. set a variable (currNum) to be equal to the current number
     * 3. set another variable (nextNum) to be equal to the next number (if there is a next number)
     * 4. check if the nextNum is greater than the current num
     * 5. if the nextNum is less than the currNum, add 1 to the counter variable, AND remove
     *    that element AND check if counter is greater than 1- if greater than 1, return false on
     *    function
    */

    // println!("find_dups result: {:?}", find_dups(this_vec));
}

// return the century when given a year. For example, 1749 will return 18
// as in the 18th centry
/*
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
fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
    let mut largest_product: i32 = input_array[0] * input_array[1];
    let mut prev_num: i32 = input_array[0];
    for i in input_array {
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

fn sort_statues(mut n: Vec<i32>) -> i32 {
    n.sort_unstable();
    n.as_slice();

    let mut curr_num: i32 = 0;
    let mut next_num: i32 = 0;
    let mut result: i32 = 0;

    let mut i: i32 = 0;
    let n_len = n.len() as i32;

    while i < (n_len - 1) {
        curr_num = n[i as usize];
        next_num = n[i as usize + 1];

        while (curr_num + 1) != (next_num) {
            result+=1;
            curr_num+=1;
        }

        i+=1;
    }
    return result;
}
*/


/*

fn find_dups(mut sequence: Vec<i32>) -> Vec<i32> {
    /*
     * 1. loop through vector
     * 2. set the current element equal to the focus variable
     * 3. remove current element from sequence vector
     * 4. run .position() function
     * 5. if .position() returns as true then add focus variable to the result vector
     * */
    let mut focus: i32 = 0;
    let mut value_at_index: usize = 0;
    let mut result: Vec<i32> = Vec::new();
    let mut i: i32 = 0;

    while i < sequence.len().try_into().unwrap() {
        focus = sequence[i as usize];
        sequence.remove(i as usize);
        value_at_index = sequence.iter().position(|&x| x == focus).unwrap();
        println!("value_at_index: {}", value_at_index);
        // if value_at_index != -1 { result.push(focus) }

        i+=1;
    }
    return result;
}

fn find_dups(sequence: Vec<i32>) -> bool {
    let sequence_len: i32 = sequence.len() as i32;
    let mut _second_to_last: i32 = sequence[sequence.len() - 2];
    let mut counter: i32 = 0;
    let mut stored_vals: Vec<i32> = Vec::new();

    let mut i: i32 = 0;
    while i <= (sequence_len - 2) {
        if stored_vals.contains(&sequence[i as usize]) {
            counter+=1;
            if counter > 1 { return false }
        }
        stored_vals.push(sequence[i as usize]);
        let mut _second_to_last: i32 = sequence[sequence.len() - 2];
        if (i + 2) < sequence_len {
            if sequence[i as usize] > sequence[i as usize + 1] { counter+=1 }
            if counter > 1 { return false }
        }
        i+=1;
    }

    if stored_vals.contains(&sequence[i as usize]) {
        counter+=1;
        if counter > 1 { return false }
    }

    if (sequence_len - 4) > 0 {
        if sequence[sequence_len as usize - 4] > sequence[sequence_len as usize - 3] {
            counter+=1;
            if counter > 1 { return false; }
        }
    }

    if sequence[sequence_len as usize - 3] > sequence[sequence_len as usize - 2] {
        counter+=1;
        if counter > 1 { return false; }
    }
    if sequence[sequence_len as usize - 2] > sequence[sequence_len as usize - 1] {
        counter+=1;
        if counter > 1 { return false; }
        }

    return counter <= 1;
}
*/

/*

fn find_dups(mut sequence: Vec<i32>) -> bool {
    let sequence_len: i32 = sequence.len() as i32;
    let mut counter: i32 = 0;
    let mut stored_vals: Vec<i32> = Vec::new();
    let mut prev_num: i32 = 0;

    let mut i: i32 = 0;
    while i <= (sequence_len - 1) {
        // check for dups
        if stored_vals.contains(&sequence[i as usize]) {
            counter+=1;
            if counter > 1 { return false }
        }
        stored_vals.push(sequence[i as usize]);

        // check to see if the current number (i) is less than the last number
        // or a value less than the first value in the vector
        if i > 0 {
            prev_num = sequence[i as usize - 1];
        }
        else {
            prev_num = sequence[i as usize] - 1;
        }

        if prev_num > sequence[i as usize] {
            counter+=1;
            if counter > 1 {
                return false
            }
            else {
                sequence.remove((sequence[i as usize] as i32).try_into().unwrap());
                find_dups(sequence);
            }
        }

        i+=1;
    }
    return counter <= 1;
}
*/
/*
fn almost_increasing_sequence(mut sequence: Vec<i32>) -> bool {
    /*
     * 1. loop through array
     * 2. look at 3 elements per check
     * 3. make sure e1 < e2 < e3
     * 4. if one of those elements violates the above then add one to the counter
     * 5. if the counter is 2 or greater, then return false else return true
    */

    let mut sequence_len = sequence.len();
    let mut counter: i32 = 0;

    if sequence_len >= 3 {
        let mut i: i32 = 0;
        let mut first: i32 = 0;
        let mut second: i32 = 0;
        let mut third: i32 = 0;

        let check_len: i32 = sequence_len as i32 - i;

        // println!("check_len: {}", check_len);

        while i < (sequence_len as i32 - 1) {
            if (sequence_len as i32 - i) >= 3 {
                first = sequence[i as usize];
                second = sequence[i as usize + 1];
                third = sequence[i as usize + 2];
                if first >= second {
                    // remove first element
                    sequence.remove(i as usize);
                    sequence_len-=1;
                    counter+=1;
                }
                else if second >= third {
                    sequence.remove(i as usize + 1);
                    sequence_len-=1;
                    counter+=1;
                }
            }
            i+=1;
        }
    }
    // println!("counter: {}", counter);
    return counter <= 1;
}
/*
 * 
*/
*/
*/

    /*
     * 1. prevNum, currNum, nextNum
     * 1.1. check if the sequence is greater than 3 number
     * 2. if prevNum < currNum < nextNum, we should be good
    */

    /*
     * 0. create a counter variable to keep track of statues removed
     * 1. loop through sequence
     * 2. set a variable (currNum) to be equal to the current number
     * 3. set another variable (nextNum) to be equal to the next number (if there is a next number)
     * 4. check if the nextNum is greater than the current num
     * 5. if the nextNum is less than the currNum, add 1 to the counter variable, AND remove
     *    that element AND check if counter is greater than 1- if greater than 1, return false on
     *    function
    */

/*
fn almost_increasing_sequence(mut _sequence: Vec<i32>) -> bool {
    let _sequence_len = _sequence.len();
    let mut _counter = 0;
    let mut _dup_count = 0;

    // check if the sequence has dups, remove them and add 1 to the counter:
    let mut k: i32 = 0;
    let mut j: i32 = 0;
    let mut curr_k_num: i32 = 0;
    while k < (_sequence_len as i32 - 1) {
        curr_k_num = _sequence[k as usize];
        while j < (_sequence_len as i32) {
            if _sequence[j as usize] == _sequence[k as usize] {
                println!("curr_k_num {}", _sequence[j as usize]);
                _dup_count+=1;
            }
            j+=1;
        }
        k+=1;
    }
    println!("dup count: {}", _dup_count);
    // check if sequence has order issues:
    if _sequence_len >= 3 {
        let mut _prev_num: i32 = 0;
        let mut _curr_num: i32 = 0;
        let mut _next_num: i32 = 0;
        let mut i: i32 = 0;
        // we want to stop at the second to last number that's why we are - 2
        while i < (_sequence_len as i32 - 1) {
            if _counter > 1 {
                return false;
            }
            if i >= 1 {
                let mut _prev_num: i32 = _sequence[i as usize - 1];
                let mut _curr_num: i32 = _sequence[i as usize];
                let mut _next_num: i32 = _sequence[i as usize + 1];
                println!("sequence[i] = {}", _curr_num);
                if (_prev_num >= _curr_num) || (_curr_num >= _next_num) {
                    _counter+=1;
                }
            }
            i+=1;
        }
    }
    return _counter <= 1;
}

fn main() {
    let this_vec = vec![1, 2, 5, 3, 5];
    almost_increasing_sequence(this_vec);
}
*/
