use clap::Parser;
use std::fs;

fn main() {

    let args = Args::parse();
    let query = &args.query;

    let contents = fs::read_to_string(args.file_path).expect("Error: file not found!");

    let lines_iter = contents.lines();

    let filter_iter = Filter {
        inner_iterator: lines_iter,
        predicate: |line: &&str| line.contains(query),
    };

    let highlight_iter = Highlight {
        inner_iterator: filter_iter,
        query,
    };

    for highlighted_line in highlight_iter {
        println!("{}", highlighted_line);
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()] 
    query: String,
    #[arg()]
    file_path: String,
}

struct Highlight<'a, I> {
    inner_iterator: I,
    query: &'a str,
}

struct Filter<I, P> {
    inner_iterator: I,
    predicate: P,
}

impl<I, P> Iterator for Filter<I, P>  
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner_iterator.next() {
                Some(item) => {
                    if (self.predicate)(&item) {
                        return Some(item);
                    }
                }
                None => return None,
            }
        }
    }
}

impl<'a, I> Iterator for Highlight<'a, I> 
where
    I: Iterator<Item = &'a str>
{
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iterator.next() {
            Some(line) => {
                if line.contains(self.query) {

                    let red_query = format!("\x1b[31m{}\x1b[0m", self.query);

                    Some(line.replace(self.query, &red_query))

                } else {
                    Some(line.to_string())
                }
            }
            None => None
        }
    }
}