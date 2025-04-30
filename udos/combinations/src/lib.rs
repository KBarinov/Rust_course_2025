#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    if k == 0 {
        ans.push(vec![]);
        return ans;
    } else if arr.is_empty() {
        return ans;
    }
    for i in 0..(arr.len() - k + 1) {
        let permutation = vec![arr[i]];
        step(arr, k, permutation.clone(), i, &mut ans)
    }
    ans
}

pub fn step(arr: &[i32], k: usize, permutation: Vec<i32>, ind: usize, ans: &mut Vec<Vec<i32>>) {
    let permutation1 = permutation.clone();
    if permutation1.len() >= k {
        ans.push((permutation1).to_vec());
    } else {
        for i in (ind + 1)..(arr.len() - (k - permutation1.len()) + 1) {
            let mut permutation2 = permutation1.clone();
            permutation2.push(arr[i]);
            step(arr, k, permutation2.clone(), i, ans)
        }
    }
}
