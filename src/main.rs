use day_05::{sum_middle_pages, sum_re_ordered_middle_pages};

fn main() {
    
    let middle_page_sum = sum_middle_pages("src/resources/puzzle.txt");
    //4766
    println!("Sum of middle pages: {middle_page_sum}");

    let re_order_middle_page_sum = sum_re_ordered_middle_pages("src/resources/puzzle.txt");
    //
    println!("Sum of middle pages: {re_order_middle_page_sum}");
}
