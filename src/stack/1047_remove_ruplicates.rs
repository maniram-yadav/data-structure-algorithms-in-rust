// 1047. Remove All Adjacent Duplicates In String


impl Solution {
    pub fn remove_duplicates(s: String) -> String {
       let mut stack:Vec<char> = Vec::new();
       for ch in s.chars() {

            if let Some(&top) = stack.last() {
                    if top==ch {
                        stack.pop();
                    } else {
                    stack.push(ch);
                    }

            } else {
                    stack.push(ch);
            }
       }
       stack.into_iter().collect()
    }
}
