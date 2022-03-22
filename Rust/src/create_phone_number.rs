pub fn create_phone_number(data: &[usize; 10]) -> String {
    format!("({}{}{}) {}{}{}-{}{}{}{}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9])
}