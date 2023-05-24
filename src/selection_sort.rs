pub fn sort<T: Ord>(list: &mut Vec<T>) {
    if list.is_empty() {
        return;
    }

    let mut sorted_list: Vec<T> = Vec::with_capacity(list.len());

    for _ in 0..list.len() {
        let smallest_index = find_smallest_index(list);
        let smallest_element = list.remove(smallest_index);
        sorted_list.push(smallest_element);
    }

    list.extend(sorted_list);
}

fn find_smallest_index<T: Ord>(list: &[T]) -> usize {
    let mut smallest = list.first().unwrap();
    let mut index: usize = 0;

    for (item_index, item) in list.iter().enumerate() {
        if item < smallest {
            smallest = item;
            index = item_index;
        }
    }

    index
}

mod tests {

    #[test]
    fn sort_list() {
        let mut list = vec![6, 3, 2, 10, 7];
        super::sort(&mut list);
        assert_eq!(vec![2, 3, 6, 7, 10], list)
    }

    #[test]
    fn empty_list_sould_be_ignored() {
        let mut list = vec![];
        super::sort(&mut list);
        let empty_list: Vec<usize> = vec![];
        assert_eq!(empty_list, list)
    }
}
