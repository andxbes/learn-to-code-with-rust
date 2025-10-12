use std::ops::Index;

#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new(String::from("Documents"));

    folder.create_file(String::from("main.rs"));
    folder.create_file(String::from("lib.rs"));
    folder.create_file(String::from("test.rs"));

    println!("{:#?}", folder);

    folder.delete_file(2);
    println!("{:#?}", folder);

    match folder.get_file(0) {
        Some(file) => println!("Retrived file: {file:?}"),
        None => println!("There as no file"),
    }
}
