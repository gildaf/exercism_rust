#[macro_export]
macro_rules! myvec {
    ( $($a:expr),*) => {
        {
            let mut myvec = Vec::new();
            $(
                myvec.push($a.to_string());
            )*
            myvec
        }
    };
}

#[macro_export]
macro_rules! mymap {
    () => {
        {
            use std::collections::HashMap;
            let _map = HashMap::new();
            _map
        }
    };
    ( $($a:expr => $b:expr),+ $(,)?) => {
        {
            use std::collections::HashMap;
            let mut _map = HashMap::new();
            $(
                _map.insert($a, $b);
            )*
            _map
        }
    };
}

// let mut myvec = Vec::new();
//             myvec.push($a);
//             myvec.push($b);
//             myvec

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test_empty() {
        let x: HashMap<usize, usize> = HashMap::new();
        assert_eq!(x, mymap!());
    }

    #[test]
    fn test_one_item() {
        let mut x: HashMap<char, usize> = HashMap::new();
        x.insert('c', 1);
        assert_eq!(x, mymap!('c' => 1));
    }

    #[test]
    fn test_two_items() {
        let mut x: HashMap<char, usize> = HashMap::new();
        x.insert('c', 1);
        x.insert('d', 2);
        assert_eq!(x, mymap!('c' => 1, 'd' => 2));
    }

    #[test]
    fn test_with_trailing() {
        let mut x: HashMap<char, usize> = HashMap::new();
        x.insert('c', 1);
        x.insert('d', 2);
        assert_eq!(x, mymap!('c' => 1, 'd' => 2, ));
    }

    // #[test]
    // fn test_only_comma_fails() {
    //     let x: HashMap<usize, usize> = HashMap::new();
    //     assert_ne!(x, mymap!(,));
    // }
}
