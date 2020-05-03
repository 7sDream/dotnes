macro_rules! count {
    ($($x:expr),*) => {
        <[()]>::len(&[$(count!(replace $x)),*])
    };
    (replace $_x:expr) => {
        ()
    }
}

macro_rules! byte_splitter {
    ($name:ident, $($x:expr),+) => {
        byte_splitter!(count!($($x),+), $name, $($x),+);
    };
    ($length:expr, $name:ident, $($x:expr),+) => {
        #[deny(const_err)]
        const _: u8 = ($($x + )+ 0 == 8) as u8 - 1;

        pub const fn $name(val: u8) -> [u8; $length] {
            let mut result = [0; $length];
            let current = 0;
            let i = 0;
            $(
                result[i] = val << current >> (8 - $x);
                #[allow(unused_variables)]
                let current = current + $x;
                #[allow(unused_variables)]
                let i = i + 1;
            )+
            result
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[allow(clippy::inconsistent_digit_grouping)] // because the byte is split to groups by spec
    fn test_byte_parser() {
        byte_splitter!(test_bit_splitter_fn, 1, 2, 3, 2);
        assert_eq!(test_bit_splitter_fn(0b_1_01_101_10), [0b1, 0b01, 0b101, 0b10]);
    }
}
