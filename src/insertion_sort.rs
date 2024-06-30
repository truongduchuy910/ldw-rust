fn insertion_sort(input: &mut [i32]) {
    for i in 1..input.len() {
        let pick = input[i];
        let mut j = i;

        while j > 0 && input[j - 1] > pick {
            input[j] = input[j - 1];
            j -= 1;
        }

        input[j] = pick;
    }
}

#[cfg(test)]
mod insertion_sort {
    use super::*;

    #[test]
    fn sort() {
        let mut input: [i32; 5] = [2,1, 4,3, 5];
        insertion_sort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn duplicate() {
        let mut input: [i32; 5] = [2,1, 4,3, 2];
        insertion_sort(&mut input);
        assert_eq!(input, [1,2, 2, 3, 4]);
    }

    #[test]
    fn empty() {
        let mut input: [i32; 0] = [];
        insertion_sort(&mut input);
        assert_eq!(input, []);
    }

    #[test]
    fn withzero() {
        let mut input: [i32; 5] = [4,3,2,0,0];
        insertion_sort(&mut input);
        assert_eq!(input, [0,0,2,3,4]);
    }

}
