use std::thread;

#[derive(Debug)]
struct Elm {
    x: i32,
    y: i32,
}

impl Elm {
    fn new(x: i32, y: i32) -> Elm {
        Elm { x, y }
    }
}

fn printElm(elm: Elm) {
    println!("{:?}", elm);
}

fn main() {
    println!("hello world!");
    thread::spawn(|| println!("hello world in spawn"))
        .join()
        .unwrap();
    //    (0..10).inspect(|x|{println!("value {}", *x) });
    let sentences = vec!["this is a sentence", "paragraphs have many sentences"];
    let words: Vec<&str> = sentences.iter().flat_map(|&x| x.split(" ")).collect();
    println!("{:?}, {:?}", sentences, words);
    let pt = Some(2).and_then(|x| Some(x + x)).and_then(|y| Some(y * y));
    println!("{:?}", pt);
    let empty = if 1 == 2 { "hello" } else { "world" };
    println!("{}", empty);
    let elm = Elm::new(12, 23);
    printElm(elm);
}
