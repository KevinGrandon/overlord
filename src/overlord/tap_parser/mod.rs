pub struct TestResult<'a> {
  passed: bool,
  description: String
}

// Parser for converting a string to a TAP `Value` instance.
// Inspired by the TOML.rs parser.
pub struct Parser<'a> {
  input: &'a str,
  pos: uint,
  plan: uint,
}

impl<'a> Parser<'a> {
  pub fn new(s: &'a str) -> Parser<'a> {
    Parser {
      input: s,
      pos: 0,
      plan: 0
    }
  }

  // Executes the parser, parsing the string contained within.
  pub fn parse(&mut self) -> Vec<TestResult> {
    let mut results = vec![];

    let plan = self.parse_plan();
    self.plan = plan;
    self.eat_whitespace();

    while !self.eof() {
      let status = self.parse_testcase_status();
      let description = self.parse_testcase_description();
      self.eat_whitespace();

      println!("Status is {} ", status);
      println!("Description is {} ", description);

      results.push(TestResult {
        passed: status,
        description: description
      });
    }

    results
  }

  /// Parse the expected plan like 1..4
  fn parse_plan(&mut self) -> uint {
    self.eat_while(|c| match c {
      '0'..'9' => true,
      _ => false
    });

    // Consume '..'
    self.eat_while(|c| match c {
      '.' => true,
      _ => false
    });

    let plan = self.eat_while(|c| match c {
      '0'..'9' => true,
      _ => false
    });

    let planOpt: Option<uint> = from_str(plan.as_slice());
    planOpt.unwrap()
  }

  /// Parse a tag or attribute name.
  fn parse_testcase_status(&mut self) -> bool {
    let status = self.eat_while(|c| match c {
      'a'..'z' | 'A'..'Z' | ' ' => true,
      _ => false
    });

    if "not ok " == status.as_slice() {
      false
    } else {
      true
    }
  }

  /// Parses a testcase result
  fn parse_testcase_description(&mut self) -> String {
    // Parse the testcase number and a dash if it exists. E.g., 4 - 
    self.eat_while(|c| match c {
      '0'..'9' | '-' | ' ' => true,
      _ => false
    });

    let mut description = String::new();

    while !self.eof() {
      let char = self.next_char();
      if char == '\n' || char == '\r' {
        break;
      }
      description.push_char(self.eat_char());
    }
    description
  }

  /// Consume and discard zero or more whitespace characters.
  fn eat_whitespace(&mut self) {
    self.eat_while(|c| c.is_whitespace());
  }

  /// Consume characters until `test` returns false.
  fn eat_while(&mut self, test: |char| -> bool) -> String {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push_char(self.eat_char());
    }
    result
  }

  /// Return the current character, and advance self.pos to the next character.
  fn eat_char(&mut self) -> char {
    let range = self.input.as_slice().char_range_at(self.pos);
    self.pos = range.next;
    range.ch
  }

  /// Read the current character without consuming it.
  fn next_char(&self) -> char {
    self.input.as_slice().char_at(self.pos)
  }

  /// Return true if all input is consumed.
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }
}

#[cfg(test)]
mod tests {
  use tap_parser::{Parser};

  #[test]
  fn parse_simple_tap() {
    let mut p = Parser::new("\
 1..4\r\n\
 ok 1 - Input file opened\r\n\
 not ok 2 - First line of the input valid\r\n\
 ok 3 - Read the rest of the file\r\n\
 not ok 4 - Summarized correctly # TODO Not written yet\r\n\
");
    assert!(p.parse().len() == 4);
  }
}
