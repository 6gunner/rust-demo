// https://leetcode-cn.com/problems/valid-parentheses/

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut stack: Vec<char> = Vec::new();
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            println!("{}, {}", i, item as char);
            let x = item as char;
            if x == '(' {
                stack.push(')');
            } else if x == '[' {
                stack.push(']');
            } else if x == '{' {
                stack.push('}');
            } else {
                if stack.is_empty() || stack.pop().unwrap() != x {
                    return false;
                }
            }
        }
        // 如果只输入"("，就去判断stack是否是空的
        return stack.is_empty();
    }
}

fn main() {
    let s = String::from("[");
    let result = Solution::is_valid(s);
    println!("is valid : {},", result);
}
