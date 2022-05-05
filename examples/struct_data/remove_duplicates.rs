struct Solutions;

impl Solutions {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let mut length = 1;
        // 利用双指针，从左往右移动， 判断数值是否相同
        loop {
            if i < j && j < nums.len() {
                if nums[i] != nums[j] {
                    if j - i > 1 {
                        nums[i + 1] = nums[j];
                    }
                    j += 1;
                    i += 1;
                    length += 1;
                } else {
                    j += 1;
                }
            } else {
                break;
            }
        }
        return length;
    }
}

fn main() {
    let mut vec: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let new_vec = Solutions::remove_duplicates(&mut vec);
    println!("{:?}", new_vec);
    println!("{:?}", vec);
}

// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/submissions/
