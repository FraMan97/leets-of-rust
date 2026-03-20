// We are given an array asteroids of integers representing asteroids in a row. 
// The indices of the asteroid in the array represent their relative position in space.

// For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.

//Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. 
// If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.
pub fn asteroid_collision(asteroids: &[i64]) -> Vec<i64> {
    let mut stack: Vec<i64> = Vec::new();

    for &asteroid in asteroids {
        let mut exploded = false;

        while let Some(&top) = stack.last() {
            if top > 0 && asteroid < 0 {
                if top.abs() < asteroid.abs() {
                    stack.pop();
                    continue; 
                } else if top.abs() == asteroid.abs() {
                    stack.pop();
                }
                
                exploded = true; 
                break;
            } else {
                break; 
            }
        }

        if !exploded {
            stack.push(asteroid);
        }
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::asteroid_collision;

    #[test]
    fn test_asteroid_collision(){
        let asteroids = [5,10,-5];
        let result = asteroid_collision(&asteroids);
        assert_eq!(result, vec![5,10]);
    }

    #[test]
    fn test_empty_asteroid_collision(){
        let asteroids = [];
        let result = asteroid_collision(&asteroids);
        assert_eq!(result, vec![]);
    }
}