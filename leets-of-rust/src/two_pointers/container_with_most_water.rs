// You are given an integer array height of length n. There are n vertical lines drawn such 
// that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container.

pub fn container_with_most_water(height: &[i32]) -> i32 {
    if height.len() < 2 {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut area = 0;

    while left < right {
        let current_area = (right - left) as i32 * std::cmp::min(height[left], height[right]);
        area = std::cmp::max(area, current_area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1; 
        }
    }
    area
}

#[cfg(test)]
mod tests {
    use super::container_with_most_water;

    #[test]
    fn test_container_with_most_water() {
        let height = [1,8,6,2,5,4,8,3,7];
        let result = container_with_most_water(&height);
        assert_eq!(result, 49);
    }

    #[test]
    fn test_empty_container_with_most_water() {
        let height = [];
        let result = container_with_most_water(&height);
        assert_eq!(result, 0);
    }
}