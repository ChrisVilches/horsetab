use std::collections::HashMap;

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
  pub fn new(sequences: &[&str]) -> Self {
    let mut result = Self {
      curr_node: 0,
      failed: false,
      graph: vec![],
      results: HashMap::new(),
    };

    result.add_node();

    for (i, seq) in sequences.iter().enumerate() {
      result.add_sequence(seq, i);
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

    let result = self.results.get(&self.curr_node).map(Vec::clone);

    if result.is_some() {
      self.reset();
    }

    result
  }

  pub fn put(&mut self, instruction: AutomataInstruction) -> Option<Vec<usize>> {
    match instruction {
      AutomataInstruction::Char(c) => {
        if let Some(child) = self.graph[self.curr_node].get(&c) {
          self.curr_node = *child;
        } else {
          self.failed = true;
        }
        self.get_current_results()
      }
      AutomataInstruction::Reset => {
        self.reset();
        None
      }
    }
  }

  fn add_sequence(&mut self, sequence: &str, id: usize) {
    let mut curr = 0;

    for c in sequence.chars() {
      #[allow(clippy::map_entry)]
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
      'R' => AutomataInstruction::Reset,
      _ => AutomataInstruction::Char(c),
    }
  }

  fn build_automata(binary_strings: &[&str]) -> SequenceAutomata {
    SequenceAutomata::new(binary_strings)
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
      let result = automata.put(instructions[i]);
      assert_eq!(result, results[i]);
    }
  }

  #[test]
  fn test_sequence_1() {
    let mut automata = build_automata(&["0101"]);
    check_results(&mut automata, "0101", &[None, None, None, Some(vec![0])]);
  }

  #[test]
  fn test_sequence_2() {
    let mut automata = build_automata(&["10", "1", "abc"]);
    check_results(
      &mut automata,
      "01Rabc",
      &[None, None, None, None, None, Some(vec![2])],
    );
  }

  #[test]
  fn test_overlap() {
    let mut automata = build_automata(&["01", "011"]);
    check_results(&mut automata, "011", &[None, Some(vec![0]), None]);
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
      "0011R011",
      &[None, None, None, None, None, None, None, Some(vec![0])],
    );
  }

  #[test]
  fn test_sequence_becomes_unreachable() {
    let mut automata = build_automata(&["0111", "011"]);
    check_results(&mut automata, "0111", &[None, None, Some(vec![1]), None]);
  }
}
