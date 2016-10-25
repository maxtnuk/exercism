pub struct PascalsTriangle{
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut self_triangle=PascalsTriangle{triangle: Vec::with_capacity(row_count as usize)};
        for i in 0..row_count{
            let mut layer_vec=Vec::new();
            layer_vec.push(1);
            if i>=2{
            for cnt in 0..i-1{
            layer_vec.push(self_triangle.triangle[i as usize-1][cnt as usize]+
                           self_triangle.triangle[i as usize-1][cnt as usize+1]);
            }}
            if i>=1{layer_vec.push(1);}
            self_triangle.triangle.push(layer_vec);
            }
        self_triangle
    }
    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
