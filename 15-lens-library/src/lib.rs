pub fn holiday_ascii_helper(text: &str) -> u8 {
    let mut hash = 0u32;

    for byte in text.bytes() {
        hash += byte as u32;
        hash *= 17;
        hash %= 256;
    }

    hash as u8
}

pub fn read_steps(reader: impl Iterator<Item = String>) -> Vec<String> {
    reader
        .flat_map(|line| {
            line.split(',')
                .map(|step| step.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holiday_ascii_helper() {
        assert_eq!(holiday_ascii_helper("HASH"), 52);
    }
}