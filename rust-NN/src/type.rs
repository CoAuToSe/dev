struct Yiota<const ROW: usize, const COL: usize> {
    items: [[f64; COL]; ROW],
}

impl<const ROW: usize, const COL: usize> Add for Yiota<ROW, COL> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut to_return = Yiota {
            items: [[0.0; COL]; ROW],
        };
        for i in 0..COL {
            for j in 0..ROW {
                to_return.items[i][j] = self.items[i][j] + rhs.items[i][j];
            }
        }
        to_return
    }
}

impl<const ROW: usize, const COL: usize> Sub for Yiota<ROW, COL> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut to_return = Yiota {
            items: [[0.0; COL]; ROW],
        };
        for i in 0..COL {
            for j in 0..ROW {
                to_return.items[i][j] = self.items[i][j] - rhs.items[i][j];
            }
        }
        to_return
    }
}
