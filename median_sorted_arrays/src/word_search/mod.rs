


fn print_spiral(array: &Vec<Vec<i32>>) {
	 let mut _top = 0;
	 let mut _bottom = 3;
	 let mut _left =0;
	 let mut _right = array.len();

	 loop{
	 	for x in _top.._right{
	 		print!("{:?} ", array[_top][x]);
	 	}
	 	_top+=1;
	 	if(_top > _bottom || _left > _right){
	 		break;
	 	}
	 	for x in _top.._bottom{
	 		print!("{:?} ", array[x][_right-1]);
	 	}
	 	if(_top > _bottom || _left > _right){
	 		break;
	 	}
	 	




	 	break;
	 }

	// add code here
}



pub fn run() {
	let  array = vec![
							vec![1,2,3],
							vec![4,5,6],
							vec![7,8,9]
						]; 
	print_spiral(&array)
	// add code here
}