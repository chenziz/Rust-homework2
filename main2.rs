pub fn sum_u32(arr: &[u32]) -> Option<u32> {
    arr.iter().try_fold(0u32,|acc,&x| acc.checked_add(x))
}

pub fn test_u32() {
    let arr1: [u32; 10] = [1,2,3,4,5,6,7,8,9,10];
    assert_eq!(Some(55), sum_u32(&arr1));
}

fn main() {
    test_u32();
}
