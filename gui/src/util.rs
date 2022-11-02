#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
///2 Dimensions
#[derive(Debug)]
pub struct Dim<T> {
    pub x: T,
    pub y: T,
}

pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<T>,
}

//Maybe this will be implemented later
#[allow(dead_code)]
impl Matrix<[u8; 3]> {
    ///Creates and allocates a matrix with specified dimensions
    pub fn dim(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            vec: Vec::with_capacity(w * h * 3),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> [u8; 3] {
        self.vec[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, val: [u8; 3]) {
        self.vec[x + y * self.width] = val;
    }

    pub fn raw(&self) -> Vec<u8> {
        //Without multithreading
        let row_length = self.width;
        let column_height = self.height;
        let mut data: Vec<u8> = Vec::with_capacity(row_length * column_height * 4);

        for y in 0..column_height {
            for x in 0..row_length {
                //Get bytes
                let bytes = self.get(x, y);

                data.push(bytes[0]);
                data.push(bytes[1]);
                data.push(bytes[2]);
                data.push(255);
            }
        }

        data
    }
}
