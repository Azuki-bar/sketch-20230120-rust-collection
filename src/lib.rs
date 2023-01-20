#[cfg(test)]
mod tests {}

mod collects {

    fn mk_string<T: std::string::ToString>(list: Vec<T>, sep: &str) -> String {
        match list.split_first() {
            Some((head, next)) => next.iter().fold(head.to_string(), |x, y| {
                x.to_string() + &sep + &y.to_string()
            }),
            None => "".to_string(),
        }
    }
    #[test]
    fn mk_string_with_val() {
        let v = vec!["1", "2", "3"];
        let res = mk_string(v, ",");
        assert_eq!(res, "1,2,3")
    }
    #[test]
    fn mk_string_with_none() {
        let v: Vec<i8> = Vec::new();
        let res = mk_string(v, ",");
        assert_eq!(res, "");
    }

    fn map<T, U>(list: Vec<T>, f: fn(T) -> U) -> Vec<U>
    where
        T: Copy,
    {
        list.iter().fold(Vec::new(), |mut x, y| {
            x.push(f(*y));
            x
        })
    }

    #[test]
    fn map_with_val() {
        let v = vec![1, 2, 3];
        let res = map(v, |x| x.to_string());
        assert_eq!(res, vec!["1", "2", "3"])
    }

    fn filter<T>(list: Vec<T>, f: fn(T) -> bool) -> Vec<T>
    where
        T: Copy,
    {
        list.iter().fold(Vec::new(), |mut x, y| {
            if f(*y) {
                x.push(*y);
                x
            } else {
                x
            }
        })
    }
}
