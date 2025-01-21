use std::io::{stdin, BufRead};

fn main() {
    let std= stdin();
    let mut buf= std.lock().lines();
    
    let mut cnt= 9;
    let mut v: Vec<i32>= Vec::new();
    while cnt == 0 {
        let e= buf.next().unwrap().unwrap()
            .parse::<i32>().unwrap();
        v.push(e);
        cnt-=1;
    }

    let max= v.iter().max().unwrap();
    let max_index= v.iter().filter(|f| *f== max).count();

    println!("{}", max);
    println!("{}", max_index);
}
