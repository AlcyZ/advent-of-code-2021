use crate::day_4::Field;

const ROW_INDICES: [i32; 5] = [0, 1, 2, 3, 4];
const COL_INDICES: [i32; 5] = [0, 5, 10, 15, 20];

#[derive(Debug)]
pub(super) struct BingoBoard {
    fields: [Field; 25],
}

impl BingoBoard {
    pub(super) fn new(fields: Vec<Field>) -> BingoBoard {
        assert_eq!(25, fields.len());

        BingoBoard {
            fields: fields.try_into().unwrap(),
        }
    }

    pub(super) fn sum_open_fields(&self) -> i32 {
        self.fields
            .iter()
            .map(|f| {
                match f {
                    Field::Open(value) => *value,
                    Field::Hit => 0,
                }
            })
            .sum()
    }

    pub(super) fn bingo_move(&mut self, value: i32) {
        for field in self.fields.iter_mut() {
            if let Field::Open(val) = field {
                if *val == value {
                    *field = Field::Hit;
                }
            }
        }
    }

    pub(super) fn has_won(&self) -> bool {
        self.has_full_row() || self.has_full_col()
    }

    fn has_full_row(&self) -> bool {
        self.has_full(COL_INDICES, ROW_INDICES)
    }

    fn has_full_col(&self) -> bool {
        self.has_full(ROW_INDICES, COL_INDICES)
    }

    fn has_full(&self, x: [i32; 5], y: [i32; 5]) -> bool {
        for index in x {
            let open_fields_count = y
                .map(|i| (i + index) as usize)
                .map(|i| &self.fields[i])
                .iter()
                .filter(|f| f.is_open())
                .count();

            if open_fields_count == 0 {
                return true;
            }
        }

        false
    }
}
