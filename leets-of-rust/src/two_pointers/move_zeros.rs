// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// Note that you must do this in-place without making a copy of the array.

pub fn move_zeros(nums: &mut [i32]) {
    let mut insert_pos = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(insert_pos, i);
            insert_pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    #[test]
    fn test_move_zeros(){
        let mut nums = vec![0,1,0,3,12];
        move_zeros(&mut nums);
        assert_eq!(nums, [1,3,12,0,0]);

    }

    #[test]
    fn test_empty_move_zeros(){
        let mut nums = vec![];
        move_zeros(&mut nums);
        assert_eq!(nums, []);

    }
}