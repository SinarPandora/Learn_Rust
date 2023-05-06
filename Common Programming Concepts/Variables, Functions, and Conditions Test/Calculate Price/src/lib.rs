pub fn calculate_price(amount: i32) -> i32 {
    if amount < 40 {
        amount * 2
    } else {
        amount
    }
}
