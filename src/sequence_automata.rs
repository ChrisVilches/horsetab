// TODO: Add more unit tests (it's already good, but there may be a few more critical corner cases.)

use std::collections::HashMap;

// TODO: Not sure if this is an automata, or just a Trie? or a tree??
//       Even if it's just a trie, it should still be called automata, because I don't want the user to
//       know the implementation details, also it might change in the future (besides, it does have other
//       things like "reset instructions" or whatever)
// TODO: I noticed that with this sequence ..........- I should be able to press many short clicks
//       before I press the long one, and it should work. But it seems that the automata is traversing
//       and getting to "not found" instead if I press too many times. I think there's something
//       fundamentally wrong here.

// TODO: A better way to implement all of this is by having the following enum.
//       { Char(ch), Reset }. So char instructions are always containing letters, and other non-char
//       instructions are defined separately. That way I also avoid the Index/IndexMut shitshow.

#[derive(Copy, Clone)]
pub enum AutomataInstruction {
  Char(char),
  Reset,
}

pub struct SequenceAutomata {
  curr_node: usize,
  failed: bool,
  graph: Vec<HashMap<char, usize>>,
  results: HashMap<usize, Vec<usize>>,
}

impl SequenceAutomata {
  // TODO: Can this be &str?
  pub fn new(sequences: &[String]) -> Self {
    let mut result = SequenceAutomata {
      curr_node: 0,
      failed: false,
      graph: vec![],
      results: HashMap::new(),
    };

    // TODO: Does it work without this?
    result.add_node();

    for (i, seq) in sequences.iter().enumerate() {
      result.add_sequence(seq, i)
    }

    result
  }

  fn reset(&mut self) {
    self.curr_node = 0;
    self.failed = false;
  }

  fn add_node(&mut self) -> usize {
    let idx = self.graph.len();
    self.graph.push(HashMap::new());
    idx
  }

  fn get_current_results(&mut self) -> Option<Vec<usize>> {
    if self.failed {
      return None;
    }

    match self.results.get(&self.curr_node) {
      Some(results) => {
        // TODO: This is fucking trash.
        let res = results.clone();
        self.reset();
        Some(res)
      }
      None => None,
    }
  }

  // TODO: Note: this executes everytime I click the mouse! So the result shouldn't be a vector because that allocs a vector in the heap,
  //       which is relatively expensive. So wrap it in an Option to avoid that.
  //       UPDATE: DONE, but verify.
  // TODO: Name "execute_input" is a bit weird, since it's not just mouse inputs, but instructions of other kinds as well.
  pub fn execute_input(&mut self, instruction: AutomataInstruction) -> Option<Vec<usize>> {
    // TODO: Maybe I could return a reference to a vector (without being inside Option). That's the cheapest
    //       way to do it I think. And being empty means None, so there's no problem checking if there were results or not.

    match instruction {
      AutomataInstruction::Char(c) => match self.graph[self.curr_node].get(&c) {
        Some(child) => {
          self.curr_node = *child;
          self.get_current_results()
        }
        None => {
          self.failed = true;
          None
        }
      },
      AutomataInstruction::Reset => {
        self.reset();
        None
      }
    }

    // TODO: The sequence must be resetted when it finds a result.

    // TODO: For now "clone", but is there a better way?
    //       Try to return the reference. That's the cheapest way to do it. No cloning, no
    //       algorithms larger than O(1). Just a few arithmetic and memory operations.
    //       The last ".map(|x| x.clone())" I added should be gone, and just return the &.
    // return self.results.get(&self.curr_node).map(|x| x.clone());
  }

  fn add_sequence(&mut self, sequence: &str, id: usize) {
    let mut curr = 0;

    for c in sequence.chars() {
      // TODO: Clippy complains, but the code given is wrong lol.
      if !self.graph[curr].contains_key(&c) {
        let v = self.add_node();
        self.graph[curr].insert(c, v);
      }

      curr = *self.graph[curr].get(&c).unwrap();
    }

    self.results.entry(curr).or_default().push(id);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn char_to_instruction(c: char) -> AutomataInstruction {
    match c {
      '0' => AutomataInstruction::Char('0'),
      '1' => AutomataInstruction::Char('1'),
      'r' => AutomataInstruction::Reset,
      _ => panic!("Wrong instruction value"),
    }
  }

  fn build_automata(binary_strings: &[&str]) -> SequenceAutomata {
    // TODO: WTF is this.
    let a: Vec<String> = binary_strings
      .iter()
      .map(|s| s.to_owned())
      .map(|c| c)
      .map(|s| s.to_owned())
      .collect();

    SequenceAutomata::new(&a)
  }

  fn check_results(
    automata: &mut SequenceAutomata,
    sequence: &str,
    results: &[Option<Vec<usize>>],
  ) {
    assert_eq!(sequence.len(), results.len());
    let instructions: Vec<AutomataInstruction> =
      sequence.chars().map(char_to_instruction).collect();
    for i in 0..sequence.len() {
      let result = automata.execute_input(instructions[i]);
      assert_eq!(result, results[i]);
    }
  }

  #[test]
  fn test_sequence_1() {
    let mut automata = build_automata(&["0101"]);
    check_results(&mut automata, "0101", &[None, None, None, Some(vec![0])]);
  }

  #[test]
  fn test_sequence_match_multiple_simultaneously() {
    let mut automata = build_automata(&["0101", "0111", "0101"]);
    check_results(&mut automata, "0101", &[None, None, None, Some(vec![0, 2])]);
  }

  #[test]
  fn test_sequence_match_twice_in_a_row() {
    let mut automata = build_automata(&["0101"]);
    check_results(
      &mut automata,
      "01010101",
      &[
        None,
        None,
        None,
        Some(vec![0]),
        None,
        None,
        None,
        Some(vec![0]),
      ],
    );
  }

  #[test]
  fn test_must_be_resetted_otherwise_wont_match() {
    let mut automata = build_automata(&["011"]);
    check_results(
      &mut automata,
      "0011r011",
      &[None, None, None, None, None, None, None, Some(vec![0])],
    );
  }

  #[test]
  fn test_sequence_becomes_unreachable() {
    let mut automata = build_automata(&["0111", "011"]);
    check_results(&mut automata, "0111", &[None, None, Some(vec![1]), None]);
  }
}
