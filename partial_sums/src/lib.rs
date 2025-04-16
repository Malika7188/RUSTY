pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let total = arr.iter().sum();
    let mut res = Vec::with_capacity(arr.len()+1);
    res.push(total);

    let mut current_sum = total;
    for &num in arr.iter().rev() {
        current_sum -= num;

        res.push(current_sum)
    }
    res
}