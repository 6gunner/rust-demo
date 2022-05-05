struct Solutions;

impl Solutions {
    pub fn move_zeroes(vec: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..=vec.len() - 1 {
            if vec[i] != 0 {
                vec[j] = vec[i];
                j += 1;
            }
        }
        for i in j..vec.len() {
            vec[i] = 0;
        }
    }
}
fn main() {
    let mut vec: Vec<i32> = vec![0, 1, 0, 3, 0, 12, 4];
    Solutions::move_zeroes(&mut vec);
    println!("{:?}", vec);
}
