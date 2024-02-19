fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    // 남은 요소들을 추가
    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }

    merged
}

pub fn merge_sort(mut v: Vec<i32>) -> Vec<i32> {
    if v.len() <= 1 {
        return v;
    }

    let mid = v.len() / 2;
    let left = merge_sort(v[..mid].to_vec());
    let right = merge_sort(v[mid..].to_vec());

    merge(left, right)
}
