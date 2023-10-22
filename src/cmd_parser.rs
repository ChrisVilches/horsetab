use crate::sequence_automata::AutomataInstruction;

fn parse_sequence(seq: &str) -> Vec<AutomataInstruction> {
  seq
    .chars()
    .map(|c| {
      if c == '.' {
        AutomataInstruction::Zero
      } else if c == '-' {
        AutomataInstruction::One
      } else {
        panic!("LOL... how to do this properly???? Maybe return a Result with error??")
      }
    })
    .collect()
}

pub struct Cmd {
  pub sequence: Vec<AutomataInstruction>,
  pub command: String,
}

impl Cmd {
  fn new(sequence: Vec<AutomataInstruction>, command: &str) -> Self {
    Self {
      sequence,
      command: command.into(),
    }
  }
}

pub fn parse_cmd(line: &str) -> Option<Cmd> {
  let line = line.trim();

  if line.is_empty() {
    return None;
  }

  let first_space = line.find(' ').expect("Should contain at least one space");
  let (seq, cmd) = line.split_at(first_space);

  Some(Cmd::new(parse_sequence(seq), cmd.trim()))
}
