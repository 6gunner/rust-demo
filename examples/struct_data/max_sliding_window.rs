// https://leetcode-cn.com/problems/sliding-window-maximum/
use std::collections::VecDeque;
struct Solution {}
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        if k == 1 {
            return nums;
        }

        // 定义一个数组存放结构
        let mut result: Vec<i32> = Vec::with_capacity(nums.len() - k as usize);

        // 定义一个排序数组，先将[0,k)个元素放入到队列里
        // 使用Vec会超出时间限制
        let mut queue: VecDeque<i32> = VecDeque::new();

        for i in 0..nums.len() {
            Solution::push(&mut queue, nums[i]);
            // println!("queue length : {:?}", queue.len());
            // 后面[k, sums.len()) 都是滑动窗口，需要记录数据
            if (i as i32) > k - 1 {
                // println!("queue length : {:?}", queue.len());
                // 需要删除掉窗口外的数据 保持queue里数据是来自窗口的;
                Solution::pop(&mut queue, nums[i - k as usize]);
                result.push(Solution::get_max(&queue));
            } else if (i as i32) == k - 1 {
                result.push(Solution::get_max(&queue));
            }
        }
        return result;
    }

    pub fn push(queue: &mut VecDeque<i32>, num: i32) {
        while !queue.is_empty() && *queue.back().unwrap() < num {
            queue.pop_back();
        }
        queue.push_back(num);
    }

    pub fn pop(queue: &mut VecDeque<i32>, num: i32) {
        if !queue.is_empty() && *queue.front().unwrap() == num {
            queue.pop_front();
        }
    }

    pub fn get_max(queue: &VecDeque<i32>) -> i32 {
        return *queue.get(0).unwrap();
    }
}

fn main() {
    let nums = vec![7, 2, 4];
    let k = 2;
    let result = Solution::max_sliding_window(nums, k);
    println!("max_sliding_window : {:?}", result);
}
