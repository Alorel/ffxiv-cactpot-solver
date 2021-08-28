use std::cmp::Ordering;
use std::fmt;
use std::ops::Index;

use super::{payouts, BoardPosition, ValuedBoardPosition};

type VBP = ValuedBoardPosition;
const NUM_ITEMS: usize = 3;

#[inline]
fn point_sum(a: VBP, b: VBP, c: VBP) -> u8 {
    a.value() + b.value() + c.value()
}

fn get_diag_row(a: VBP, b: VBP, c: VBP) -> DiagRow {
    let mut rows = [a, b, c];
    rows.sort_by(|a, b| a.position().col().cmp(&b.position().col()));

    let p1 = rows[0].position();
    let p2 = rows[1].position();
    let p3 = rows[2].position();

    if p1.col() != 0 || p2.col() != 1 || p2.row() != 1 || p3.col() != 2 {
        return DiagRow::None;
    }

    if p1.row() == 0 {
        if p3.row() == 2 {
            return DiagRow::TopLeftBottomRight;
        }
    } else if p3.row() == 0 {
        return DiagRow::BottomLeftTopRight;
    }

    DiagRow::None
    // let mut rows = [a, b, c];
    // rows.sort_by(|a, b| a.position().col().cmp(&b.position().col()));
    //
    // let p1 = &rows[0].position();
    // let p2 = &rows[1].position();
    // let p3 = &rows[2].position();
    //
    // if p1.col() != 0 || p2.col() != 1 || p2.row() != 1 || p3.col() != 2 {
    //     return DiagRow::None;
    // }
    //
    // if p1.row() == 0 {
    //     if p3.row() == 2 {
    //         return DiagRow::TopLeftBottomRight;
    //     }
    // } else if p3.row() == 0 {
    //     return DiagRow::BottomLeftTopRight;
    // }
    //
    // DiagRow::None
}

#[derive(Debug, Clone, Eq)]
pub struct EndRow {
    items: [ValuedBoardPosition; NUM_ITEMS],
    payout_value: u16,
    point_sum: u8,
    diag_row: DiagRow,
}

impl PartialEq for EndRow {
    #[inline]
    fn eq(&self, other: &EndRow) -> bool {
        self.items == other.items
    }
}

impl Ord for EndRow {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.payout_value.cmp(&other.payout_value)
    }
}

impl PartialOrd for EndRow {
    #[inline]
    fn partial_cmp(&self, other: &EndRow) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiagRow {
    BottomLeftTopRight,
    TopLeftBottomRight,
    Both,
    None,
}

impl EndRow {
    pub fn new(a: ValuedBoardPosition, b: ValuedBoardPosition, c: ValuedBoardPosition) -> Self {
        let point_sum = point_sum(a, b, c);

        Self {
            items: [a, b, c],
            diag_row: get_diag_row(a, b, c),
            point_sum,
            payout_value: payouts::payout_for_points(point_sum),
        }
    }

    fn compare_col_row<F>(&self, it: F) -> Option<u8>
    where
        F: Fn(&BoardPosition) -> u8,
    {
        let p1 = it(&self[0].position());
        let p2 = it(&self[1].position());
        let p3 = it(&self[2].position());

        match p3 == p1 && p3 == p2 {
            true => Some(p1),
            false => None,
        }
    }

    #[inline]
    pub fn get_column(&self) -> Option<u8> {
        self.compare_col_row(BoardPosition::col)
    }

    #[inline]
    pub fn get_row(&self) -> Option<u8> {
        self.compare_col_row(BoardPosition::row)
    }

    #[inline]
    pub fn payout_value(&self) -> u16 {
        self.payout_value
    }

    #[inline]
    pub fn has_column(&self, col: u8) -> bool {
        self[0].position().col() == col
            || self[1].position().col() == col
            || self[2].position().col() == col
    }

    #[inline]
    pub fn diag_row(&self) -> DiagRow {
        self.diag_row
    }
}

impl Index<usize> for EndRow {
    type Output = ValuedBoardPosition;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index <= NUM_ITEMS, "Index out of bounds");
        &self.items[index]
    }
}

impl fmt::Display for EndRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self[0].fmt(f)?;
        f.write_str(", ")?;
        self[1].fmt(f)?;
        f.write_str(", ")?;
        self[2].fmt(f)
    }
}

#[cfg(test)]
mod test {
    use std::slice::Iter;

    use super::*;
    use smallvec::SmallVec;

    fn std_vbp(val_a: u8, val_b: u8, val_c: u8) -> [VBP; 3] {
        [
            VBP::from_pos(val_a, BoardPosition::default()),
            VBP::from_pos(val_b, BoardPosition::default()),
            VBP::from_pos(val_c, BoardPosition::default()),
        ]
    }

    #[test]
    fn get_diag_row() {
        type Spec = (VBP, VBP, VBP, DiagRow);
        fn mkspec(a: [u8; 2], b: [u8; 2], c: [u8; 2], exp: DiagRow) -> Spec {
            (
                VBP::from_u8(1, a[0], a[1]),
                VBP::from_u8(1, b[0], b[1]),
                VBP::from_u8(1, c[0], c[1]),
                exp,
            )
        }
        let specs = [
            mkspec([0, 0], [1, 1], [2, 2], DiagRow::TopLeftBottomRight),
            mkspec([0, 2], [1, 1], [2, 0], DiagRow::BottomLeftTopRight),
            mkspec([0, 2], [1, 1], [2, 1], DiagRow::None),
            mkspec([0, 0], [1, 1], [2, 0], DiagRow::None),
            mkspec([0, 2], [1, 1], [1, 0], DiagRow::None),
            mkspec([0, 2], [0, 1], [2, 0], DiagRow::None),
            mkspec([0, 2], [1, 0], [2, 0], DiagRow::None),
            mkspec([1, 2], [1, 1], [2, 0], DiagRow::None),
        ];
        let iter: Iter<Spec> = specs.iter();

        for (a, b, c, exp) in iter {
            let row = super::get_diag_row(*a, *b, *c);
            assert_eq!(
                exp,
                &row,
                "{} | {}  {}",
                a.position(),
                b.position(),
                c.position()
            );
        }
    }

    #[test]
    fn get_column() {
        type Spec = (u8, u8, u8, Option<u8>);
        let specs = [
            (1, 1, 1, Some(1)),
            (0, 1, 1, None),
            (1, 0, 1, None),
            (0, 0, 1, None),
            (0, 0, 0, Some(0)),
        ];
        let iter: Iter<Spec> = specs.iter();
        for (a, b, c, expectation) in iter {
            let row = {
                let a = VBP::from_u8(1, *a, 1);
                let b = VBP::from_u8(1, *b, 1);
                let c = VBP::from_u8(1, *c, 1);

                EndRow::new(a, b, c)
            };

            assert_eq!(*expectation, row.get_column(), "{}, {}, {}", a, b, c);
        }
    }

    #[test]
    fn point_sum() {
        let [a, b, c] = std_vbp(9, 4, 7);

        let res = super::point_sum(a, b, c);
        assert_eq!(res, 20);
    }

    #[test]
    fn payout_value() {
        let [a, b, c] = std_vbp(1, 2, 3);
        let [d, e, f] = std_vbp(9, 7, 4);

        let r1 = EndRow::new(a, b, c);
        let r2 = EndRow::new(d, e, f);

        assert_eq!(10000, r1.payout_value(), "EndRow 1");
        assert_eq!(306, r2.payout_value(), "EndRow 2");
    }

    #[test]
    fn has_column() {
        fn mkrow(pos_1: u8, pos_2: u8, pos_3: u8) -> EndRow {
            let p: SmallVec<[VBP; 3]> = [pos_1, pos_2, pos_3]
                .iter()
                .map(|col| VBP::from_pos(1, BoardPosition::new(*col, 0)))
                .collect();

            EndRow::new(*p.get(0).unwrap(), *p.get(1).unwrap(), *p.get(2).unwrap())
        }

        type Spec = (EndRow, u8, bool);
        let specs = [
            (mkrow(0, 1, 2), 0, true),
            (mkrow(1, 0, 2), 0, true),
            (mkrow(2, 1, 0), 0, true),
            (mkrow(1, 1, 1), 0, false),
        ];
        let iter: Iter<Spec> = specs.iter();

        for (row, col, exp) in iter {
            assert_eq!(row.has_column(*col), *exp, "{}", row);
        }
    }

    #[test]
    fn ord() {
        let [a, b, c] = std_vbp(1, 2, 3);
        let [d, e, f] = std_vbp(9, 7, 4);
        let [g, h, i] = std_vbp(6, 3, 7);

        let r1 = EndRow::new(a, b, c); // 10000
        let r2 = EndRow::new(d, e, f); // 306
        let r3 = EndRow::new(g, h, i); // 72

        let mut arr = [&r2, &r1, &r3];
        arr.sort_unstable();

        assert_eq!([&r3, &r2, &r1], arr);
    }

    #[test]
    fn index() {
        let [a, b, c] = std_vbp(1, 2, 3);
        let r = EndRow::new(a, b, c);

        assert_eq!(r[0], a, "a");
        assert_eq!(r[1], b, "b");
        assert_eq!(r[2], c, "c");
    }
}
