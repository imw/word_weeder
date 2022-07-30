use std::fs;
use std::collections::HashMap;
use std::env;

struct WordCounter {
    counter: HashMap<String,i32>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let phrase = &args[2];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");


    let mut pre_counter = WordCounter::new();
    let mut post_counter = WordCounter::new();
    let mut in_abstract: bool = false;
    let mut abs = Vec::new();
    for line in contents.lines() {
        if  line.starts_with("AB") {
            in_abstract = true;
        } else if !line.starts_with("      "){
            in_abstract = false;
        }
        if in_abstract {
            let ll = line.to_ascii_lowercase();
            abs.push(ll);
        }

    }

    let abs = abs.join(" ");

    let llocs : Vec<usize> = abs.match_indices(phrase).map(|(i, _)|i).collect();
    let mut prevl = 0;
    for i in 0..llocs.len(){
        let pre = abs[prevl..llocs[i]].to_string();
        let pre_words = pre.split_whitespace();
        for word in pre_words.rev().take(3) {
            pre_counter.add(word);
        }
        prevl = llocs[i];
    }

    let rlocs : Vec<usize> = abs.rmatch_indices(phrase).map(|(i, _)|i).collect();
    let mut prevr = abs.len();
    for i in 0..rlocs.len(){
        let post = abs[rlocs[i]..prevr].to_string();
        let post_words = post.split_whitespace();
        for word in post_words.take(3) {
            post_counter.add(word);
        }
        prevr = rlocs[i];
    }
    pre_counter.print("pre");
    println!("");
    post_counter.print("post");
}

impl WordCounter {
    fn new() -> WordCounter {
        let counter = HashMap::new();
        return WordCounter{counter}
    }

    fn add(self: &mut Self, to_add: &str){
        let mut owned_str = to_add.to_owned();
        owned_str.retain(|c| !r#"[]{}(),".;:'"#.contains(c));
        match self.counter.get_mut(&owned_str) {
            Some(v) => { *v = *v + 1;}
            None => {self.counter.insert(owned_str,1);}
        }
            
    }

    fn print(self: &mut Self,name: &str) {
        println!("{}: {:?}",name, self.counter)
    }
}
