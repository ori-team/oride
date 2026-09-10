pub fn parse_data(raw: &str) -> Result<u32, std::num::ParseIntError> {
    raw.trim().parse::<u32>()
}
