const PAYOUTS_ARRAY: [u16; 25] = [
    0u16, 0, 0, 0, 0, 0, 10000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306,
    1080, 144, 1800, 3600,
];

pub fn payout_for_points(points: u8) -> u16 {
    match points > 24 {
        true => 0,
        false => PAYOUTS_ARRAY[points as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn std_values() {
        for i in 0u8..25 {
            let exp = *PAYOUTS_ARRAY.get(i as usize).unwrap();
            let actual = payout_for_points(i);
            assert_eq!(exp, actual, "For {} points", i);
        }
    }

    #[test]
    fn other_u8_values() {
        for i in 25..u8::MAX {
            assert_eq!(0, payout_for_points(i), "For {} points", i);
        }
    }
}
