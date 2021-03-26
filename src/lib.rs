use std::iter::Iterator;

struct DirectionIter {
    start_pos: (usize, usize),
    limits: (usize, usize),
    deltavector: Vec<(isize, isize)>,
}
impl DirectionIter {
    /// Todo store all direction vectors
    /// store x,y start tuple
    pub fn new(start: (usize, usize), limit: (usize, usize)) -> Self {
        DirectionIter {
            start_pos: start,
            limits: limit,
            deltavector: vec![(0, 1), (0, -1), (-1, 0), (1, 0)],
        }
    }

    //Todo create constructor for 4 ans 8 directions
}

impl Iterator for DirectionIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        while let Some((dx, dy)) = self.deltavector.pop() {
            let x = self.start_pos.0 as isize + dx;
            let y = self.start_pos.1 as isize + dy;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                if x < self.limits.0 && y < self.limits.1 {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut it = super::DirectionIter::new((0, 0), (10, 10));
        let ans: Vec<(usize, usize)> = it.collect();
        assert_eq!(ans, vec![ (1, 0) , (0, 1)]);

        let mut it = super::DirectionIter::new((10, 10), (100, 100));
        let ans: Vec<(usize, usize)> = it.collect();
        assert_eq!(ans, vec![(11, 10), (9, 10), (10, 9), (10, 11), ]);
    }
}


