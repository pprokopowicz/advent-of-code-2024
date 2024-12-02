pub fn solve(input: &Vec<Vec<usize>>) {
    let result = input
        .iter()
        .filter(|nums| {
            if is_ascending_or_descending(nums) {
                return false;
            }

            are_diffs_correct(nums)
        })
        .count();

    println!("Part 1 solution: {}", result);
}

fn is_ascending_or_descending(nums: &Vec<usize>) -> bool {
     let mut reverse_nums = nums.clone();
     reverse_nums.reverse();

     !nums.is_sorted() && !reverse_nums.is_sorted()
}

fn are_diffs_correct(nums: &Vec<usize>) -> bool {
     for index in 0..(nums.len() - 1) {
          let difference = nums[index].abs_diff(nums[index + 1]);

          if difference < 1 || difference > 3 {
              return false;
          }
      }

      return true;
}