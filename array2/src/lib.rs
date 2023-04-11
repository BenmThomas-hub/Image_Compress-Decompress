#[derive(Clone)]

pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T: Clone> Array2<T> {

    // Returns the value at a specified index in a 2d array in a 1d vector
    pub fn get(&self, row:usize, col:usize) -> &T {
        // find how to row and column and return desired element in 1D vector
        &self.data[(row * self.width) + col]
    }

    // Returns the value at a specified index if there is one, if not returns None
    fn get_index(&self, column: usize, row: usize) -> Option<&T> {
        //if column < self.width && row < self.height {
        if row * self.width + column < self.data.len(){
            Some(&self.data[row * self.width + column])
        }
        else {
            None
        }
    }
    
    // Returns a vector of all 4 pixel chunks in the image
    pub fn get_chunks(&self) -> Vec<Vec<T>>{
        let mut chunks = vec![];
        for i in 0..self.height/2{
            for j in 0..self.width/2{
                chunks.push(self.iterate_square(j*2, i*2));
            }
        }
        return chunks;
    }
    
    // Given a specific index, returns a vector of the items in a 2x2 grid with the index as the top left corner
    fn iterate_square(&self, col: usize, row:usize) -> Vec<T>{
        let mut pixels: Vec<T> = vec![];
        for i in row..(row+2){
            for j in col..(col+2){
                pixels.push(self.get(i as usize, j as usize).clone());
            }
        }
        return pixels;
    }

    // Sets a 2x2 square of values in an Array2 that was created initially
    pub fn set_square(&mut self, col: usize, row:usize, pixels: Vec<T>) -> (){
        let mut count = 0;
        for i in row..(row+2){
            for j in col..(col+2){
                self.set_index(i as usize, j as usize, pixels[count].clone());
                count += 1;
            }
        }
    }

    // Returns the height of the Array2
    pub fn get_height(&self) -> usize{
        (&self.height).clone()
    }

    // Returns the width of the Array2
    pub fn get_width(&self) -> usize{
        (&self.width).clone()
    }

    // Sets a specific index of the Array2 to the data inputted
    pub fn set_index(&mut self, row:usize, col:usize, data: T) -> () {
        self.data[(row * self.width) + col] = data;
    }

    // Returns an iterator over the entire Array2 going through every value in the row then moving onto the next
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.height).flat_map(move |r| (0..self.width).map(move |c| (c, r, self.get_index(c, r).unwrap())))
    }

    // Returns an iterator over the entire Array2 going through every value in the column then moving onto the next
    pub fn iter_column_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width).flat_map(move |c| (0..self.height).map(move |r| (c, r, self.get_index(c, r).unwrap())))
    }

    // Creates an Array2 using a vector of data in row major form
    pub fn from_row_major(width: usize, height: usize, data: Vec<T>) -> Self{
        Array2{
            width,
            height,
            data,
        }
    }

    // Creates an Array2 using a vector of data in column major form
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

    //C reates an Array2 containing a copied single value
    pub fn single_val(width: usize, height: usize, data: T) -> Self{
        Array2{
            width,
            height,
            data: vec![data; width*height]
        }
    }
}