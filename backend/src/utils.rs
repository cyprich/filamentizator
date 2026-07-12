pub trait MaxStringLengthTrait {
    fn apply_max_string_length(&mut self, max_length: usize);
}

pub fn max_string_length(val: &str, max_length: usize) -> String {
    // number of dots, which will represent truncaded string
    const N: usize = 2;

    if val.chars().count() <= max_length {
        return val.to_string();
    }

    if max_length <= N {
        return ".".repeat(max_length);
    }

    // TODO: what happens here with bigger utf-8 characters tho
    format!("{}{}", &val[..max_length - N], ".".repeat(N))
}
