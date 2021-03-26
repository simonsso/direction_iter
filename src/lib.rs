use std::iter::Iterator;

struct DirectionIter {}
impl DirectionIter {

    /// Todo store all direction vectors 
    /// store x,y start tuple
    pub fn new( start:(usize,usize), limit:(usize,usize) ) -> Self {
        DirectionIter {}
    }

    //Todo create constructor for 4 ans 8 directions
}

impl Iterator for DirectionIter {
    type Item = (usize,usize);

    fn next(&mut self) -> Option<(usize,usize)> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut it = super::DirectionIter::new((0,0),(10,10));
        let ans:Vec<(usize,usize)> = it.collect();
        assert_eq!(ans,vec![(0,1),(1,0)]);

        let mut it = super::DirectionIter::new((10,10),(100,100));
        let ans:Vec<(usize,usize)> = it.collect();
        assert_eq!(ans,vec![(10,11),(10,9),(9,10),(11,10)]);
    }
}

//  THIS is the code I will replace
// if (0..matrix.len()).contains(&(x-1)) && (0..matrix[0].len()).contains(&y) {
//     if matrix[x][y] <= matrix[x-1][y-0] {
//         q.push_back((x,y))
//     }
// }
// if (0..matrix.len()).contains(&(x+1)) && (0..matrix[0].len()).contains(&y) {

// }
// if (0..matrix.len()).contains(&x) && (0..matrix[0].len()).contains(&(y-1)) {

// }
// if (0..matrix.len()).contains(&x) && (0..matrix[0].len()).contains(&(y+1)) {

// }     