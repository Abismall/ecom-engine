use super::model::Discount;

pub fn sort_discounts_by_value_asc(discounts: &mut Vec<Discount>) {
    discounts.sort_by(|a, b| a.value.cmp(&b.value));
}

pub fn sort_discounts_by_value_desc(discounts: &mut Vec<Discount>) {
    discounts.sort_by(|a, b| b.value.cmp(&a.value));
}

pub fn sort_discounts_by_start_date_asc(discounts: &mut Vec<Discount>) {
    discounts.sort_by(|a, b| a.start_date.cmp(&b.start_date));
}

pub fn sort_discounts_by_start_date_desc(discounts: &mut Vec<Discount>) {
    discounts.sort_by(|a, b| b.start_date.cmp(&a.start_date));
}

pub fn calculate_orderline_total(quantity: i32, price: i32) -> i32 {
    quantity * price
}

pub fn calculate_discount_amount(orderline_total: i32, discount: &Discount) -> i32 {
    match discount.discount_type.as_str() {
        "percentage" => (orderline_total as f32 * (discount.value as f32 / 100.0)).round() as i32,
        "fixed" => discount.value,
        _ => 0,
    }
}
