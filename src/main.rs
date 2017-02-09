extern crate rand;
use rand::Rng;

mod shell;

trait Sortable<T: Copy + Ord> {
    fn shuffle(&mut self, rnd: &mut rand::ThreadRng);
    fn is_sorted(&self) -> bool;
}

impl <T: Copy + Ord> Sortable<T> for [T] {
    fn shuffle(&mut self, rnd: &mut rand::ThreadRng) {
        for i in 0..self.len()-1 {
            let ind = rnd.next_u64() % self.len() as u64;
            self.swap(i, ind as usize);
        }
    }

    fn is_sorted(&self) -> bool {
        for i in 0..self.len()-1 {
            if self[i] > self[i+1] {
                return false;
            }
        }

        true
    }
}

fn create_arr(min: i32, max: i32, count: usize) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();

    for _ in 0..count {
        let val = rand::thread_rng().gen_range(min, max);
        arr.push(val);
    }

    arr
}

fn main() {
    let mut arr: Vec<i32> = create_arr(-100, 100, 20);

    let seq: [u32; 5] = [9, 5, 3, 2, 1];
    shell::shellsort(&mut arr, &seq[..]);

    println!("arr {}", if arr.is_sorted() { "is sorted" } else { "not sorted!" });
}
