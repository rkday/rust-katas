fn binary_chop_recursive(value: u64, array: &Vec<u64>) -> i32 {
    let mut array2 = array.clone();
    binary_chop_recursive_int(0, value, &mut array2)
}

fn binary_chop_recursive_int(arraypos: i32, value: u64, array: &mut Vec<u64>) -> i32 {
    match array.len() {
        0 => -1,
        1 => if array[0] == value { arraypos } else { -1 },
        _ => {
            let halfway_point = array.len() / 2;
            if array[halfway_point] <= value {
                let mut right = array.split_off(halfway_point);
                binary_chop_recursive_int(arraypos + (array.len() as i32), value, &mut right)
            } else {
                array.split_off(halfway_point);
                binary_chop_recursive_int(arraypos, value, array)
            }
        }
    }
}

fn binary_chop_iterative(value: u64, array: &Vec<u64>) -> i32 {
    if array.len() == 0 {
        return -1;
    }

    let mut L = 0;
    let mut R = array.len() - 1;
    while R >= L {
        let mut middle = (L + R) / 2;
        let current_val = array[middle];
        if current_val == value {
            return middle as i32;
        } else if current_val < value {
            L = middle + 1;
        } else if current_val > value {
            R = middle - 1
        }
    }
    return -1;
}


fn binary_chop_iterative2(value: u64, array: &Vec<u64>) -> i32 {
    let mut array2 = array.clone();
    let mut index: i32 = 0;
    let num_iterations = (array.len() as f64).log2() as u32;
    for x in 0..num_iterations {
        let halfway_point = array2.len() / 2;
        let mut right = array2.split_off(halfway_point);
        println!("Iteration {} of {}: left is {:?}, right is {:?}", x, num_iterations, array2, right);
        if right[0] <= value {
            array2 = right;
            index += halfway_point as i32;
        }
    }
    index
}


#[cfg(test)]
mod tests {
    use binary_chop_recursive;
    use binary_chop_iterative;

    #[test]
    fn recursive_test() {
        binary_chop_test(&binary_chop_recursive);
        duplicates(&binary_chop_recursive);
    }

    #[test]
    fn iterative_test() {
        binary_chop_test(&binary_chop_iterative);
        duplicates2(&binary_chop_iterative);
    }

    fn binary_chop_test<F>(binary_chop_func: &F)
                       where F: Fn(u64, &Vec<u64>) -> i32 {
        correct_locations(binary_chop_func);
        missing_locations(binary_chop_func);
        empty(binary_chop_func);
    }
 
    fn correct_locations<F>(binary_chop_func: &F)
                       where F: Fn(u64, &Vec<u64>) -> i32 {
        let v = vec![0, 100, 150, 170, 190, 203, 204];
        assert_eq!(binary_chop_func(0, &v), 0);
        assert_eq!(binary_chop_func(100, &v), 1);
        assert_eq!(binary_chop_func(150, &v), 2);
        assert_eq!(binary_chop_func(170, &v), 3);
        assert_eq!(binary_chop_func(190, &v), 4);
        assert_eq!(binary_chop_func(203, &v), 5);
        assert_eq!(binary_chop_func(204, &v), 6);
    }

    fn missing_locations<F>(binary_chop_func: &F)
                       where F: Fn(u64, &Vec<u64>) -> i32 {
        let v = vec![0, 100, 150, 170, 190, 203, 204];
        assert_eq!(binary_chop_func(8, &v), -1);
        assert_eq!(binary_chop_func(300, &v), -1);
    }

    fn empty<F>(binary_chop_func: &F)
                       where F: Fn(u64, &Vec<u64>) -> i32 {
        let v = vec![];
        assert_eq!(binary_chop_func(8, &v), -1);
        assert_eq!(binary_chop_func(300, &v), -1);
    }


    fn duplicates<F>(binary_chop_func: &F)
                       where F: Fn(u64, &Vec<u64>) -> i32 {
        let v = vec![10, 20, 20, 30];
        assert_eq!(binary_chop_func(20, &v), 2);
        assert_eq!(binary_chop_func(30, &v), 3);
        let v = vec![10, 15, 20, 20, 30];
        assert_eq!(binary_chop_func(20, &v), 3);
        assert_eq!(binary_chop_func(30, &v), 4);
    }

    fn duplicates2<F>(binary_chop_func: &F)
                       where F: Fn(u64, &Vec<u64>) -> i32 {
        let v = vec![10, 20, 20, 30];
        assert_eq!(binary_chop_func(20, &v), 1);
        assert_eq!(binary_chop_func(30, &v), 3);
        let v = vec![10, 15, 20, 20, 30];
        assert_eq!(binary_chop_func(20, &v), 2);
        assert_eq!(binary_chop_func(30, &v), 4);
    }
}
