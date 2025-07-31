
pub struct Matrix {
    // Implement your Matrix struct
    rows: Vec<Vec<u32>>
}

impl Matrix {
    pub fn new(input: &str) -> Self {

        let mut rows = vec![];
        input
            .lines()
            .for_each( |v| {
                let row = v
                                .split(" ")
                                .map(|x| x.parse::<u32>().unwrap())
                                .collect::<Vec<_>>();
                rows.push(row);
            });
        Self {rows}
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        let row_no = row_no - 1;
        if row_no >= self.rows.len() {
            None
        } else {

            Some(self.rows[row_no].clone())
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
    let col_no = col_no - 1;
        
        if col_no >= self.rows[0].len() {
            None
        } else {
            let mut col = vec![];
            self.rows.iter().for_each(|row| col.push(row[col_no]));
            Some(col)
        }
    }
}
