fn first_n_zeroes(data: &md5::Digest) -> u8 {
    let mut n = 0;

    for &i in data.iter() {
        // hi nibble
        if (i & 0xf0) != 0 { return n; } else { n += 1; }
        // lo nibble
        if (i & 0x0f) != 0 { return n; } else { n += 1; }
    }

    n
}

pub fn run(input: &str, threshold: u8) -> u128 {
    let mut i = 0u128;
    loop {
        let check = format!("{}{}", input, i);
        let digest = md5::compute(&check);
        if first_n_zeroes(&digest) >= threshold { break i; } else { i += 1; }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case("abcdef609043"; "case 1")]
    #[test_case("pqrstuv1048970"; "case 2")]
    fn exact(input: &str) {
        let digest = md5::compute(input);
        let count = first_n_zeroes(&digest);

        println!("{} zeroes -- {:x}", count, digest);

        assert!(count >= 5);
    }


    #[test_case("abcdef",   609043 as u128; "case 1")]
    #[test_case("pqrstuv", 1048970 as u128; "case 2")]
    fn values(input: &str, result: u128) {
        assert_eq!(run(input, 5), result);
    }
}