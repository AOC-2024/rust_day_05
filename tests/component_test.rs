use day_05::{sum_middle_pages, sum_re_ordered_middle_pages};


#[test]
fn should_sum_middle_pages() {
    assert_eq!(sum_middle_pages("tests/resources/puzzle.txt"), 143);
}

#[test]
fn should_sum_re_ordered_middle_pages() {
    assert_eq!(sum_re_ordered_middle_pages("tests/resources/puzzle.txt"), 123);
}