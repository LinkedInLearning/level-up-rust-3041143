fn main() {
    let list = vec![1, 4, 4, 55, 55, 32, 1];
    let a = uniq(list);
    println!("{a:?}")
}

fn uniq<T: PartialEq>(vec: Vec<T>) -> Vec<T> {
    let mut tmp: Vec<T> = Vec::new();
    for el in vec {
        if tmp.contains(&el) {
            continue;
        }
        tmp.push(el);
    }
    tmp
}