mod sort;

#[cfg(test)]
mod sort_tests {
    use rand::Rng;

    use crate::sort::nlogn::*;
    use crate::sort::nsq::*;

    fn gen_test_case() -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let n = 10;
        let mut vec: Vec<i32> = Vec::with_capacity(n);

        for _ in 0..n {
            let r = rng.gen();
            vec.push(r);
        }

        vec
    }

    fn test_sort(sort_fn: fn(Vec<i32>) -> Vec<i32>) {
        for _ in 0..1000 {
            let mut v = gen_test_case();

            let my_ans = sort_fn(v.clone());
            v.sort();

            assert_eq!(v, my_ans);
        }
    }

    #[test]
    fn test_bubble() {
        test_sort(bubble);
    }

    #[test]
    fn test_insertion() {
        test_sort(insertion);
    }

    #[test]
    fn test_selection() {
        test_sort(selection);
    }

    #[test]
    fn test_merge() {
        test_sort(merge_sort);
    }
}
