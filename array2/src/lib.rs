#[derive(Clone)]

pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T: Clone> Array2<T> {

    pub fn get(&self, row:usize, col:usize) -> &T {
        // find how to row and column and return desired element in 1D vector
        &self.data[(row * self.width) + col]
    }

    fn get_index(&self, column: usize, row: usize) -> Option<&T> {
        //if column < self.width && row < self.height {
        if row * self.width + column < self.data.len(){
            Some(&self.data[row * self.width + column])
        }
        else {
            None
        }
    }
    
    pub fn get_chunks(&self) -> impl Iterator<Item = Vec<T>>{
        let mut chunks = vec![];
        for i in 0..self.height/2{
            for j in 0..self.width/2{
                chunks.push(self.iterate_square(j*2, i*2));
            }
        }
        return chunks.into_iter();
    }
    
    fn iterate_square(&self, col: usize, row:usize) -> Vec<T>{
        let mut pixels: Vec<T> = vec![];
        for i in row..row+1{
            for j in col..col+1{
                pixels.push(self.get(i as usize, j as usize).clone());
            }
        }
        return pixels;
    }


    pub fn set_square(&mut self, col: usize, row:usize, pixels: Vec<T>) -> (){
        let mut count = 0;
        for i in row..row+1{
            for j in col..col+1{
                self.set_index(i as usize, j as usize, pixels[count].clone());
                count += 1;
            }
        }
    }

    pub fn get_height(&self) -> usize{
        (&self.height).clone()
    }

    pub fn get_width(&self) -> usize{
        (&self.width).clone()
    }

    pub fn set_index(&mut self, row:usize, col:usize, data: T) -> () {
        self.data[(row * self.width) + col] = data;
    }
    
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.height).flat_map(move |r| (0..self.width).map(move |c| (c, r, self.get_index(c, r).unwrap())))
    }

    pub fn iter_column_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width).flat_map(move |c| (0..self.height).map(move |r| (c, r, self.get_index(c, r).unwrap())))
    }

    pub fn from_row_major(width: usize, height: usize, data: Vec<T>) -> Self{
        Array2{
            width,
            height,
            data,
        }
    }

    pub fn from_col_major(width: usize, height: usize, data: Vec<T>) -> Self{
        let x = Array2{
            width,
            height,
            data,
        };
        let mut vec = vec![];
        for (_r, _c, d) in x.iter_column_major(){
            vec.push(d.clone());
        }
        Array2{
            width,
            height,
            data: vec,
        }
    }

    pub fn single_val(width: usize, height: usize, data: T) -> Self{
        Array2{
            width,
            height,
            data: vec![data; width*height]
        }
    }
}