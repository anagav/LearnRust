
fn print_paranthesis(n:i32, open: i32, close: i32, result: String) {
	if close == n{
		println!("{:?}", result);
	}	

	if open < n{
		print_paranthesis(n, open+1, close, result.to_string()+"{");		
	}

	if open > close{
		print_paranthesis(n, open, close+1, result.to_string()+"}");
	}
}

fn main() {
    print_paranthesis(3, 0, 0, "".to_string())
}
