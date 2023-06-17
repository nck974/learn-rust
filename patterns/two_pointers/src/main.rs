// Given an array of sorted numbers and a target sum, find a pair in the array whose sum is equal to the given target.
// Input: [1, 2, 3, 4, 6], target=6
// Output: [1, 3]

const TARGET: i32 =  6;

fn target_sum(v: Vec<i32>, target: i32) -> Vec<usize>{
    let mut index_1 = 0;
    let mut index_2 = v.len() - 1;
    loop {
        if index_1 == index_2{
            break;
        }
        let sum = v[index_1] + v[index_2];
        if sum == target {
            return vec!(index_1, index_2);
        }else if sum < target{
            index_1 += 1;
        }else{
            index_2 -= 1;
        }
    }
    unreachable!();

}

fn main() {
    let v = vec![1, 2, 3, 4, 6];
    dbg!(&v);
    dbg!(target_sum(v, TARGET));    

}
