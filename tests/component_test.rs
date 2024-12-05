use day_05::sum_middle_pages;


#[test]
fn should_sum_middle_pages() {
    assert_eq!(sum_middle_pages("tests/resources/puzzle.txt"), 143);
}