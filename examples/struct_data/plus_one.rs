struct Solutions;

impl Solutions {
    pub fn plus_one(arr: &mut Vec<i32>) -> Vec<i32> {
        let mut i = arr.len() - 1;
        loop {
            if arr[i] < 9 {
                arr[i] += 1;
                return arr.to_vec();
            }
            arr[i] = 0;
            if i > 0 {
                i -= 1;
            } else {
                break;
            }
        }

        let mut new_arr = vec![0; arr.len() + 1];
        println!("{:?}", &new_arr);
        new_arr[0] = 1;
        return new_arr;
    }
}

fn main() {
    let mut vec: Vec<i32> = vec![9, 9, 9];
    let new_vec = Solutions::plus_one(&mut vec);
    println!("{:?}", new_vec);
}
