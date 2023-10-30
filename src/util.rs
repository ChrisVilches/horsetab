pub fn clean_command_lines<'a, I>(lines: I) -> Vec<String>
where
  I: Iterator<Item = &'a str>,
{
  // TODO: The line regarding comments was removed (in order to allow the "shebang")
  //       was this OK?? review. HOPEFULLY UNIT TEST.
  lines
    .map(|line| line.trim().to_owned())
    .filter(|line| !line.is_empty())
    .filter(|line| line.starts_with("#!") || !line.starts_with('#'))
    .collect()
}

pub fn effectful_format_bytes_merge_newlines(string: &mut [u8], n: usize, prev_char: u8) -> &[u8] {
  let mut j = 0;
  let mut last_was_newline = false;
  for i in 0..n {
    if string[i] == b'\n' {
      if !last_was_newline {
        string[j] = b'\n';
        j += 1;
      }
      last_was_newline = true;
    } else {
      string[j] = string[i];
      j += 1;
      last_was_newline = false;
    }
  }

  let leading_extra_newline = n > 0 && prev_char == string[0] && prev_char == b'\n';
  let start = usize::from(leading_extra_newline);

  &string[start..j]
}

pub fn parse_shebang_or_default(text: &str, default: &[&str]) -> Vec<String> {
  let trimmed = text.trim();

  if trimmed.starts_with("#!") {
    trimmed.split('\n').next().unwrap()[2..]
      .split(' ')
      .filter(|s| !s.is_empty())
      .map(std::borrow::ToOwned::to_owned)
      .collect()
  } else {
    // TODO: I don't want to use two maps with to_owned.
    default
      .iter()
      .map(std::borrow::ToOwned::to_owned)
      .map(std::borrow::ToOwned::to_owned)
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("hello\nworldxxx", 11, b'a', "hello\nworld")]
  #[test_case("\nhello\n\nworld", 13, b'\n', "hello\nworld")]
  #[test_case("\n\n", 0, b'\n', "")]
  #[test_case("\n", 1, b'\n', "")]
  #[test_case("\n\n", 1, b'\n', "")]
  #[test_case("\n\n", 1, b'f', "\n")]
  #[test_case("\n\n", 2, b'\n', "")]
  #[test_case("\n\n", 2, b'x', "\n")]
  #[test_case("hello\n\n", 6, b'f', "hello\n")]
  #[test_case("hello\n\n", 7, b'f', "hello\n")]
  #[test_case("hello\n\nx", 8, b'\n', "hello\nx")]
  fn test_effectful_format_merge_newlines(input: &str, len: usize, prev_char: u8, expected: &str) {
    let mut bytes: Vec<u8> = input.chars().map(|c| c as u8).collect();
    let slice = bytes.as_mut_slice();

    let result = effectful_format_bytes_merge_newlines(slice, len, prev_char);
    assert_eq!(String::from_utf8_lossy(result), expected);
  }

  #[test_case("#!/bin/zsh -c\nhello, some text", &["/usr/bin/env", "python3", "-c"], &["/bin/zsh", "-c"])]
  #[test_case("   #!/bin/zsh -c\nhello world", &["/usr/bin/env", "python3", "-c"], &["/bin/zsh", "-c"])]
  #[test_case("   #!  /bin/zsh -c\nhello", &["/usr/bin/env", "python3", "-c"], &["/bin/zsh", "-c"])]
  #[test_case("   #!  /bin/zsh  -c  \nworld", &["/usr/bin/env", "python3", "-c"], &["/bin/zsh", "-c"])]
  #[test_case(" # /bin/zsh  -c \n default", &["/usr/bin/env", "python3", "-c"], &["/usr/bin/env", "python3", "-c"])]
  #[test_case(" empty ", &["/usr/bin/env", "python3", "-c"], &["/usr/bin/env", "python3", "-c"])]
  #[test_case("hello, some text", &["bash", "-c"], &["bash", "-c"])]
  fn test_parse_shebang_or_default(text: &str, default: &[&str], expected: &[&str]) {
    assert_eq!(parse_shebang_or_default(text, default), expected);
  }
}
