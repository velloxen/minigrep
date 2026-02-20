// `main` defines what happens when the end executable is run
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pattern = &args[1];
    let file_name = &args[2];

    let file_contents = std::fs::read_to_string(file_name).unwrap();

    let lines = file_contents.lines();
    let res = lines.filter(|l| l.contains(pattern));

    let res_string = itertools::join(res, "\n");
    println!("{}", res_string);
}
