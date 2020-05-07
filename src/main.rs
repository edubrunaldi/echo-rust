use std::env;

fn help() {
  print!("\
    Usage: echo [SHORT-OPTION]... [STRING]...\n\
  ");
  print!("\
    Echo the STRING(s) to standard output.\n\
    \n\
    -n             do not output the trailing newline\n\
  ");
  print!("\
    -e             enable interpretation of backslash escapes\n\
    -E             disable interpretation of backslash escapes (default)\n"
  );
  print!("\
    \n\
    If -e is in effect, the following sequences are recognized:\n\
    \n\
  ");
  print!(
    "\
    \\\\      backslash\n\
    \\a      alert (BEL)\n\
    \\b      backspace\n\
    \\c      produce no further output\n\
    \\e      escape\n\
    \\f      form feed\n\
    \\n      new line\n\
    \\r      carriage return\n\
    \\t      horizontal tab\n\
    \\v      vertical tab\n\
  ");
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0); // remove first element (path to the program)
  // flags
  let mut newline: bool = true;
  let mut interpretation_backslash: bool = false;

  while let Some(word) = args.get(0) {
    if word.as_str() == "-n" {
      newline = false;
    } else if word.as_str() == "-e" {
      interpretation_backslash = true;
    } else if word.as_str() == "-help" || word.as_str() == "-h" {
      help();
      return;
    } else {
      break;
    }
    args.remove(0);
  }

  for arg in args {
    if !interpretation_backslash {
      print!("{} ", arg);
    } else {
      // rust do not support vertical tab (\v) and others, so it was add using unicode
      // https://github.com/rust-lang/rfcs/issues/751
      match arg.as_str() {
        "\\a" => print!("\u{0007}"),
        "\\b" => print!("\u{0008}"),
        "\\c" => break,
        "\\e" => print!("\u{001B}"),
        "\\f" => print!("\u{C}"),
        "\\n" => print!("\n"),
        "\\r" => print!("\r"),
        "\\t" => print!("\t"),
        "\\v" => print!("\u{000B}"),
        _ => print!("{} ", arg),
      }
    }
  }
  if newline {
    print!("\n");
  }
}
