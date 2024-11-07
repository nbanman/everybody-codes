pub mod coord;

#[macro_export]
macro_rules! include_input {
    ($year:literal / $day:literal / $part:literal) => {{
        include_str!(concat!(
            "../../../inputs/20",
            stringify!($year),
            "/y",
            stringify!($year),
            "d",
            stringify!($day),
            "-",
            stringify!($part),
            ".txt"
        ))
    }};
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
