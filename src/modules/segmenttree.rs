pub struct SegmentTree<T: Clone> {
    pub values : Vec<T>,
    ln : usize,
    zero : T,
    cmpf : fn(&T, &T)->T,
}

impl<T: Clone>  SegmentTree<T> {
    pub fn new( array : &Vec<T>, zero : T, cmpf : fn(&T, &T)->T) -> SegmentTree<T> 
    {
        let mut ln : usize = 1;

        while( ln < array.len() ) { ln *= 2; }
        let mut values : Vec<T> =vec![zero.clone(); ln-1];
        let mut blanks : Vec<T> =vec![zero.clone(); ln-array.len()];

        values.extend(array.clone());
        values.extend(blanks);

        for i in (0..ln-1).rev() {
            values[i] = cmpf(&values[i*2+1], &values[i*2+2]);
        }

        SegmentTree { values, ln, zero, cmpf }
    }

    pub fn fromOneValue( no : usize, zero : &T, cmp : fn(&T,&T)-> T) -> SegmentTree<T> {
        let array : Vec<T> = vec![zero.clone(); no];
        SegmentTree::new(&array, zero.clone(), cmp)
    }
}

impl<T: Clone> SegmentTree<T> {
    pub fn update( &mut self, target_idx : usize, value : T) {
        let mut idx = target_idx+self.ln-1;

        self.values[idx] = value;
        while ( idx > 0 ) {
            idx = (idx-1)/2;
            let l : usize = idx*2+1;
            let r : usize = idx*2+2;
            self.values[idx] = (self.cmpf)(&self.values[l], &self.values[r]);
        }
    }

    // [left ~ right)
    fn _find( &self, left : usize, right : usize, node : usize, nowl : usize, nowr : usize ) -> T{
        if ( nowl >= left && nowr <= right ) { return self.values[node].clone(); }
        if ( nowr <= left || nowl >= right ) { return self.zero.clone(); }
        let mid = (nowl+nowr)/2;

        let ans: T = (self.cmpf)(&self._find(left, right, node*2+1, nowl, mid ), &self._find(left, right, node*2+2, mid, nowr));
        ans
    }

    pub fn find( &self, left : usize, right : usize ) -> T{
        self._find(left, right, 0, 0, self.ln)
    }
}

