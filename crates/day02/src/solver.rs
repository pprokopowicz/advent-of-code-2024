pub fn solve(input: &Vec<Vec<usize>>, skips: usize) -> usize {
    let result = input
        .iter()
        .filter(|nums| are_diffs_correct(nums, skips))
        .count();

    result
}

fn are_diffs_correct(nums: &Vec<usize>, skips: usize) -> bool {
    let mut current_skips = 0;
    let mut is_increasing_option = Option::None;

    for index in 0..(nums.len() - 1) {
        match is_increasing_option {
            Some(is_increasing) => {
                if is_increasing {
                    if nums[index + 1] < nums[index] {
                        if current_skips < skips {
                            current_skips += 1;
                        } else {
                            return false;
                        }
                    }
                } else {
                    if nums[index + 1] > nums[index] {
                        if current_skips < skips {
                            current_skips += 1;
                        } else {
                            return false;
                        }
                    }
                }
            }
            None => {
                if nums[index] == nums[index + 1] {
                    if current_skips < skips {
                        current_skips += 1;
                    } else {
                        return false;
                    }
                } else {
                    is_increasing_option = Some(nums[index + 1] > nums[index])
                }
            }
        }

        let difference = nums[index].abs_diff(nums[index + 1]);

        if difference < 1 || difference > 3 {
            if current_skips < skips {
                current_skips += 1;
            } else {
                return false;
            }
        }
    }

    return true;
}
