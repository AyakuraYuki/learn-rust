pub mod bitwise {
    pub fn set_bit(attr: u64, position: i32, flag: bool) -> u64 {
        if position <= 0 {
            return attr;
        }
        if flag {
            attr | 1u64 << (position - 1)
        } else {
            attr & !(1u64 << (position - 1))
        }
    }

    pub fn set_bits(mut attr: u64, positions: &Vec<i32>, flag: bool) -> u64 {
        if positions.len() == 0 {
            return attr;
        }
        for position in positions {
            if *position <= 0 {
                continue;
            }
            attr = set_bit(attr, *position, flag)
        }
        attr
    }

    pub fn get_bit(attr: u64, position: i32) -> i8 {
        ((attr >> (position - 1)) & 1) as i8
    }

    pub fn get_bit_flag(attr: u64, position: i32) -> bool {
        get_bit(attr, position) == 1
    }

    pub fn get_total_bits(flags: &Vec<i8>) -> u64 {
        let mut attr = 0u64;
        if flags.len() == 0 {
            return attr;
        }
        for (i, flag) in flags.iter().enumerate() {
            attr = set_bit(attr, (i + 1) as i32, *flag != 0)
        }
        attr
    }

    pub fn get_bits(attr: u64, length: usize) -> Vec<i8> {
        let mut bits = Vec::with_capacity(length);
        for i in 1..=length {
            let b = get_bit(attr, i as i32) as i8;
            bits.push(b);
        }
        bits
    }
}

#[cfg(test)]
mod test {
    use crate::bitwise::{get_bit, get_bit_flag, get_bits, get_total_bits, set_bit, set_bits};

    #[test]
    fn test_set_bit() {
        // case 0: 不对 0 位操作
        let attr = set_bit(0, 0, true);
        assert_eq!(0u64, attr);

        // case 1: 加位
        let attr = set_bit(0, 1, true);
        assert_eq!(1u64, attr);

        // case 2: 减位
        let mut attr = 143u64;
        attr = set_bit(attr, 3, false);
        assert_eq!(139u64, attr);

        // case 3: 加多位
        let mut attr = 143u64;
        attr = set_bit(attr, 3, true);
        attr = set_bit(attr, 12, true);
        attr = set_bit(attr, 15, true);
        attr = set_bit(attr, 16, true);
        assert_eq!(51343u64, attr);

        // case 4: 减多位
        let mut attr = 143u64;
        attr = set_bit(attr, 3, false);
        attr = set_bit(attr, 8, false);
        assert_eq!(11u64, attr);
    }

    #[test]
    fn test_set_bits() {
        let mut attr_a = 0u64;
        attr_a = set_bit(attr_a, 2, true);
        attr_a = set_bit(attr_a, 3, true);
        attr_a = set_bit(attr_a, 4, true);
        attr_a = set_bit(attr_a, 5, true);
        attr_a = set_bit(attr_a, 6, true);
        attr_a = set_bit(attr_a, 7, true);

        let positions = vec![2, 3, 4, 5, 6, 7];
        let attr_b = set_bits(0, &positions, true);

        assert_eq!(attr_a, attr_b);
    }

    #[test]
    fn test_get_bit() {
        let tests = vec![
            // attr: u64, position: i32, want: i8
            (0u64, 1i32, 0i8),
            (1u64, 1i32, 1i8),
            (139u64, 3i32, 0i8),
            (51343u64, 3i32, 1i8),
            (11u64, 2i32, 1i8),
        ];
        for (attr, position, want) in tests {
            let get = get_bit(attr, position);
            assert_eq!(get, want);
        }
    }

    #[test]
    fn test_get_bit_flag() {
        let tests = vec![
            // attr: u64, position: i32, want: bool
            (0u64, 1i32, false),
            (1u64, 1i32, true),
            (139u64, 3i32, false),
            (51343u64, 3i32, true),
            (11u64, 2i32, true),
        ];
        for (attr, position, want) in tests {
            let get = get_bit_flag(attr, position);
            assert_eq!(get, want);
        }
    }

    #[test]
    fn test_get_total_bits() {
        let flags: Vec<i8> = vec![0, 1, 0, 1];
        let attr = get_total_bits(&flags);
        assert_eq!(10u64, attr);
    }

    #[test]
    fn test_get_bits() {
        let attr = 1109u64;
        let length: usize = 15;
        let want: Vec<i8> = vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0];
        let get = get_bits(attr, length);
        assert_eq!(get, want);
    }
}
