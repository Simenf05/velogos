use std::cell::RefCell;
use std::io;
use std::fs;
use std::rc::Rc;
use std::rc::Weak;
use std::str::Chars;
use std::vec;

use rand::seq::IndexedRandom;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub letter: char,
}

impl Node {
    pub fn new(file_name: String) -> Result<Rc<RefCell<Node>>, io::Error> {

        let root = Rc::new(RefCell::new(Node { children: Vec::new(), parent: None, letter: '\0' }));
        Self::file_handle(root.clone(), file_name)?;

        return Ok(root);
    }

    fn file_handle(root: Rc<RefCell<Node>>, file_name: String) -> Result<(), io::Error> {

        let file = fs::read_to_string(file_name)?;
        let split = file.split("\n");

        for word in split {
            let chars = word.chars();
            Self::append_chars(root.clone(), chars)?;
        }
        
        Ok(())
    }

    fn append_chars(parent: Rc<RefCell<Node>>, mut chars: Chars<'_>) -> Result<(), io::Error> {
        let letter_opt = chars.next();
        if letter_opt.is_none() {
            return Ok(())
        }

        let letter = letter_opt.unwrap();

        let mut parent_borrow = parent.borrow_mut();
        let children = &parent_borrow.children;
        let child= children.iter().find(|child| child.borrow().letter == letter);

        if child.is_some() {
            Self::append_chars(child.unwrap().clone(), chars)?;
            return Ok(());
        };

        let this_node = Rc::new(RefCell::new(Node {parent: Some(Rc::downgrade(&parent)), children: vec!(), letter: letter}));
        parent_borrow.children.push(this_node.clone());

        Self::append_chars(this_node, chars)?;
        return Ok(());
    }

    pub fn gen_word(&self) -> String {
        let child = self.children.choose(&mut rand::rng());
        if child.is_none() {
            return String::from(self.letter);
        }
        let mut word = child.unwrap().borrow().gen_word();
        if self.letter != '\0' {
            word.insert(0, self.letter);
        }
        word
    }

    #[allow(dead_code)]
    pub fn gen_word_with(&self, include: char) -> String {
        loop {
            let word = self.gen_word();
            if word.contains(include) {
                return word;
            }
        }
    }

    #[allow(dead_code)]
    pub fn walk(&self, f: &dyn Fn(&Node), bubble: bool) {
        if bubble {
            self.children.iter().for_each(|child| child.borrow().walk(f, bubble));
            f(self);
        }
        else {
            f(self);
            self.children.iter().for_each(|child| child.borrow().walk(f, bubble));
        }
    }
}