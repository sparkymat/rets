pub fn range_position_string(value: &String, start_pos: usize, end_pos: usize) -> Option<String> {
    if start_pos > value.len() || end_pos > value.len() || start_pos >= end_pos {
        return None;
    }
    let mut position_string = String::with_capacity(value.len());
    for (i, _ch) in value.chars().enumerate() {
        if i == start_pos {
            position_string.push('└');
        } else if i == end_pos - 1 {
            position_string.push('┘');
        } else if (i > start_pos) && (i < end_pos - 1) {
            position_string.push('─');
        } else {
            position_string.push(' ');
        }
    }

    return Some(position_string);
}
