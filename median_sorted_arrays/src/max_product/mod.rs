
use std;

fn max_product_subarray(stack: &mut Vec<i32>)  {
	let mut cur_max = 1;
	let mut max = 0;
	for element in stack {
		cur_max = std::cmp::max(1, cur_max * (*element));
		max = std::cmp::max(cur_max, max);
	}
	println!("{:?}", max);
}


pub fn run() {
		let mut stack:Vec<i32> = vec![1,2,3,-4,5,-6];
		max_product_subarray(&mut stack);
}
