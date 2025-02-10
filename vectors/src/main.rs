fn main() {
let mut lang: Vec<String> = Vec::new();
lang.push(String::from("C"));
lang.push(String::from("Java"));
lang.push(String::from("Python"));
println!("Programming languages are: {:?} ",lang);
let remove_lastone = lang.pop();
println!("Removed last one is: {:?} ", remove_lastone);
// let third = &lang[2];

let third = lang.get(2);

println!("The third is: {:?} ",third);

let nums = vec![5, 3, 1, 4, 2];

println!("The numbers vector is: {:?} ", nums);


}
