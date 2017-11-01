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

#[cfg(test)]
mod tests {
    use binary_chop_recursive;

    #[test]
    fn correct_locations() {
        let v = vec![0, 100, 150, 170, 190, 203, 204];
        assert_eq!(binary_chop_recursive(0, &v), 0);
        assert_eq!(binary_chop_recursive(100, &v), 1);
        assert_eq!(binary_chop_recursive(150, &v), 2);
        assert_eq!(binary_chop_recursive(170, &v), 3);
        assert_eq!(binary_chop_recursive(190, &v), 4);
        assert_eq!(binary_chop_recursive(203, &v), 5);
        assert_eq!(binary_chop_recursive(204, &v), 6);
    }

    #[test]
    fn missing_locations() {
        let v = vec![0, 100, 150, 170, 190, 203, 204];
        assert_eq!(binary_chop_recursive(8, &v), -1);
        assert_eq!(binary_chop_recursive(300, &v), -1);
    }

    #[test]
    fn duplicates() {
        let v = vec![10, 20, 20, 30];
        assert_eq!(binary_chop_recursive(20, &v), 2);
        assert_eq!(binary_chop_recursive(30, &v), 3);
        let v = vec![10, 15, 20, 20, 30];
        assert_eq!(binary_chop_recursive(20, &v), 3);
        assert_eq!(binary_chop_recursive(30, &v), 4);
    }


}
