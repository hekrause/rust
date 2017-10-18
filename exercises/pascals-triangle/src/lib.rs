use std::iter::repeat;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut master_vec: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);

        if row_count > 0 {
            master_vec.push(vec![0, 1, 0]);

            for i in 1..row_count as usize {
                let mut vec: Vec<u32> = repeat(0).take(i + 3).collect();

                for j in 1..((i + 3) - 1) {
                    vec[j] = master_vec[i - 1][j - 1] + master_vec[i - 1][j]
                }

                master_vec.push(vec);
            }

            master_vec.iter_mut().for_each(|x| x.retain(|&y| y != 0));
        }

        PascalsTriangle { rows: master_vec }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

