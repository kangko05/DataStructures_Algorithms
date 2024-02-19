pub fn bubble(mut v: Vec<i32>) -> Vec<i32> {
    let len = v.len();

    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }

    v
}

pub fn insertion(mut v: Vec<i32>) -> Vec<i32> {
    let len = v.len();

    for i in 1..len {
        let key = v[i];
        let mut j = i;

        // 현재 요소(key)를 정렬된 부분의 올바른 위치에 삽입합니다.
        // 이를 위해, key보다 큰 모든 요소를 오른쪽으로 한 칸씩 이동시킵니다.
        while j > 0 && v[j - 1] > key {
            v[j] = v[j - 1];
            j -= 1;
        }
        // key를 적절한 위치에 삽입합니다.
        v[j] = key;
    }

    v
}

pub fn selection(mut v: Vec<i32>) -> Vec<i32> {
    let len = v.len();

    for i in 0..len {
        let mut min_idx = i;

        for j in i..len {
            if v[j] < v[min_idx] {
                min_idx = j;
            }
        }

        v.swap(i, min_idx);
    }

    v
}
