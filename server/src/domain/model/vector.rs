#[allow(unused)]
pub fn first<T: Clone>(s: &[T]) -> Option<T> {
    s.get(0).cloned()
}

#[cfg(test)]
mod test_vector {
    #[test]
    fn test_first() {
        // expect Some
        let list: Vec<u8> = vec![1, 2, 3, 4, 5];
        let list_opt: Option<u8> = super::first(&list);
        assert!(list_opt.is_some());
        if let Some(first_number) = list_opt {
            assert_eq!(list[0], first_number);
        }

        // expect None
        let list: Vec<u8> = Vec::new();
        assert!(super::first(&list).is_none());
    }
}
