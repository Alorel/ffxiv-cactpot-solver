use std::cmp::Ordering;
use std::fmt;
use std::ops::Index;

use super::{BoardPosition, payouts, ValuedBoardPosition};

type VBP = ValuedBoardPosition;

#[derive(Debug)]
pub struct EndRow {
    payout_value: u16,
    point_sum: u8,
    diag_row: DiagRow,
    items: [&'static ValuedBoardPosition; 3],
}

impl Eq for EndRow {}

impl PartialEq<EndRow> for EndRow {
    fn eq(&self, other: &EndRow) -> bool {
        self.items == other.items
    }
}

impl PartialOrd<EndRow> for EndRow {
    fn partial_cmp(&self, other: &EndRow) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for EndRow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.payout_value.cmp(&other.payout_value)
    }
}

fn point_sum(a: &VBP, b: &VBP, c: &VBP) -> u8 {
    a.value() + b.value() + c.value()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiagRow {
    BottomLeftTopRight,
    TopLeftBottomRight,
    Both,
    None,
}

fn get_diag_row(a: &VBP, b: &VBP, c: &VBP) -> DiagRow {
    let mut rows = [a, b, c];
    rows.sort_by(|a, b| a.position().col().cmp(&b.position().col()));

    let p1 = &rows[0].position();
    let p2 = &rows[1].position();
    let p3 = &rows[2].position();

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
}

impl EndRow {
    pub fn new(
        a: &'static ValuedBoardPosition,
        b: &'static ValuedBoardPosition,
        c: &'static ValuedBoardPosition,
    ) -> Self {
        let items = [a, b, c];

        let point_sum = point_sum(&a, &b, &c);
        let payout_value = payouts::payout_for_points(point_sum);

        Self {
            items,
            diag_row: get_diag_row(&a, &b, &c),
            point_sum,
            payout_value,
        }
    }

    fn compare_col_row(&self, it: fn(&BoardPosition) -> u8) -> Option<u8> {
        let p1 = it(&self[0].position());
        let p2 = it(&self[1].position());
        let p3 = it(&self[2].position());

        match p3 == p1 && p3 == p2 {
            true => Some(p1),
            false => None,
        }
    }

    pub fn get_column(&self) -> Option<u8> {
        self.compare_col_row(|p| p.col())
    }

    pub fn get_row(&self) -> Option<u8> {
        self.compare_col_row(|p| p.row())
    }

    pub fn payout_value(&self) -> u16 {
        self.payout_value
    }

    pub fn has_column(&self, col: u8) -> bool {
        self[0].position().col() == col
            || self[1].position().col() == col
            || self[2].position().col() == col
    }

    pub fn diag_row(&self) -> DiagRow {
        self.diag_row
    }
}

impl Index<usize> for EndRow {
    type Output = ValuedBoardPosition;

    fn index(&self, index: usize) -> &Self::Output {
        self.items.get(index).unwrap()
    }
}

impl fmt::Display for EndRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod test {
    use std::slice::Iter;

    use super::*;

    fn std_vbp(val_a: u8, val_b: u8, val_c: u8) -> [&'static VBP; 3] {
        [
            VBP::from_pos(val_a, BoardPosition::default()),
            VBP::from_pos(val_b, BoardPosition::default()),
            VBP::from_pos(val_c, BoardPosition::default()),
        ]
    }

    #[test]
    fn get_diag_row() {
        type Spec = (&'static VBP, &'static VBP, &'static VBP, DiagRow);
        fn mkspec(a: (u8, u8), b: (u8, u8), c: (u8, u8), exp: DiagRow) -> Spec {
            (
                VBP::from_u8(1, a.0, a.1),
                VBP::from_u8(1, b.0, b.1),
                VBP::from_u8(1, c.0, c.1),
                exp,
            )
        }
        let specs = [
            mkspec((0, 0), (1, 1), (2, 2), DiagRow::TopLeftBottomRight),
            mkspec((0, 2), (1, 1), (2, 0), DiagRow::BottomLeftTopRight),
            mkspec((0, 2), (1, 1), (2, 1), DiagRow::None),
            mkspec((0, 0), (1, 1), (2, 0), DiagRow::None),
            mkspec((0, 2), (1, 1), (1, 0), DiagRow::None),
            mkspec((0, 2), (0, 1), (2, 0), DiagRow::None),
            mkspec((0, 2), (1, 0), (2, 0), DiagRow::None),
            mkspec((1, 2), (1, 1), (2, 0), DiagRow::None),
        ];
        let iter: Iter<Spec> = specs.iter();

        for (a, b, c, exp) in iter {
            let row = &super::get_diag_row(a, b, c);
            assert_eq!(
                exp,
                row,
                "{}, {}, {}",
                &a.position(),
                &b.position(),
                &c.position()
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

        let res = super::point_sum(&a, &b, &c);
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
        fn mkrow<'p>(pos_1: u8, pos_2: u8, pos_3: u8) -> EndRow {
            let p: Vec<&'static VBP> = [pos_1, pos_2, pos_3]
                .iter()
                .map(|col| {
                    let pos = BoardPosition::new(*col, 0);
                    VBP::from_pos(1, pos)
                })
                .collect();

            EndRow::new(*p.get(0).unwrap(), *p.get(1).unwrap(), *p.get(2).unwrap())
        }

        type Spec<'a> = (EndRow, u8, bool);
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
        arr.sort();

        assert_eq!([&r3, &r2, &r1], arr);
    }

    #[test]
    fn index() {
        let [a, b, c] = std_vbp(1, 2, 3);
        let r = EndRow::new(a, b, c);

        assert_eq!(&r[0], a, "a");
        assert_eq!(&r[1], b, "b");
        assert_eq!(&r[2], c, "c");
    }
}
