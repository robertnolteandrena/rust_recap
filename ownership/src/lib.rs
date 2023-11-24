pub fn get_string_len(string: String) -> usize {
    string.len()
}
pub fn get_string_len_borrowed(string: &String) -> usize {
    string.len()
}
pub fn get_sin(value: f32) -> f64 {
    value.sin() as f64
}
