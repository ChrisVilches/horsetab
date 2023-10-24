pub fn clean_command_lines<'a, I>(lines: I) -> Vec<String>
where
  I: Iterator<Item = &'a str>,
{
  lines
    .map(|line| line.trim().to_owned())
    .filter(|line: &String| !line.is_empty())
    .filter(|line| !line.starts_with('#'))
    .collect()
}
