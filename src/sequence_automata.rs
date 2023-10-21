// TODO: This can be unit tested, so do it.

use std::{cell::RefCell, rc::Rc};

use crate::constants::MouseClickKind;

// TODO: Not sure if this is an automata, or just a Trie? or a tree??
// TODO: I noticed that with this sequence ..........- I should be able to press many short clicks
//       before I press the long one, and it should work. But it seems that the automata is traversing
//       and getting to "not found" instead if I press too many times. I think there's something
//       fundamentally wrong here.

struct Node {
  result_id: Vec<usize>,
  children: [Option<Rc<RefCell<Node>>>; 2],
}

impl Node {
  fn new() -> Self {
    Self {
      result_id: vec![],
      children: [None, None],
    }
  }

  pub fn has_children(&self) -> bool {
    self.children[0].is_some() || self.children[1].is_some()
  }
}

pub struct SequenceAutomata {
  main_node: Rc<RefCell<Node>>,
  curr_node: Rc<RefCell<Node>>,
}

impl SequenceAutomata {
  pub fn new() -> Self {
    let main_node = Rc::new(RefCell::new(Node::new()));

    Self {
      main_node: Rc::clone(&main_node),
      curr_node: Rc::clone(&main_node),
    }
  }

  fn reset_sequence(&mut self) {
    self.curr_node = Rc::clone(&self.main_node);
  }

  // TODO: Note: this executes everytime I click the mouse! So the result shouldn't be a vector because that allocs a vector in the heap,
  //       which is relatively expensive. So wrap it in an Option to avoid that.
  //       UPDATE: DONE, but verify.
  // TODO: Name "execute_input" is a bit weird, since it's not just mouse inputs, but instructions of other kinds as well.
  pub fn execute_input(&mut self, click_kind: MouseClickKind) -> Option<Vec<usize>> {
    match click_kind {
      MouseClickKind::Reset => {
        self.reset_sequence();
        return None;
      }
      _ => {}
    }

    if !self.curr_node.borrow().has_children() {
      self.reset_sequence();
      // TODO: Warning, this can cause infinite recursion.
      //       It will cause it if there's no nodes.
      //       Implement differently please.
      self.execute_input(click_kind);
      return None;
    }

    // TODO: This is a bit ugly.
    let child_idx = match click_kind {
      MouseClickKind::Short => 0,
      _ => 1,
    };

    let mut next_curr: Option<Rc<RefCell<Node>>> = None;
    let mut found = false;

    {
      let curr_node_ref = self.curr_node.borrow();
      let children = &curr_node_ref.children;
      let next_child = &children[child_idx];

      match next_child {
        Some(child) => {
          found = true;
          next_curr = Some(Rc::clone(child));
        }
        None => {}
      }
    }

    if found {
      // TODO: (all of this) is bad
      self.curr_node = next_curr.unwrap();
      // TODO: For now "clone", but is there a better way?
      return Some(self.curr_node.borrow().result_id.clone());
    }

    // TODO: This is printed more than expected. Maybe it's bugged.
    println!("Not found");
    self.reset_sequence();
    None
  }

  pub fn add_sequence(&mut self, sequence: Vec<MouseClickKind>, id: usize) {
    let mut curr: Rc<RefCell<Node>> = Rc::clone(&self.main_node);

    for item in sequence {
      // TODO: A bit ugly.
      let child_idx = match item {
        MouseClickKind::Short => 0,
        _ => 1,
      };

      RefCell::borrow_mut(&curr).children[child_idx]
        .get_or_insert_with(|| Rc::new(RefCell::new(Node::new())));

      // TODO: What a shit show.
      let mut next_curr: Option<Rc<RefCell<Node>>> = None;

      {
        let curr_ref = curr.borrow();
        let children = &curr_ref.children;

        let next_child = &children[child_idx];

        if let Some(child) = next_child {
          next_curr = Some(Rc::clone(&child));
        }
      }

      curr = next_curr.unwrap();
    }

    RefCell::borrow_mut(&curr).result_id.push(id);
  }
}
