use std::fmt::Debug;

pub fn shellsort<T: Copy + Ord + Debug>(arr: &mut [T], seq: &[u32]) {
    for inc in seq {

        let inc = *inc as usize;
        for i in inc..arr.len() {

            let mut j = i;
            while j >= inc && arr[j] < arr[j-inc] {
                arr.swap(j, j-inc);
                j -= inc;
            }
        }
    }
}