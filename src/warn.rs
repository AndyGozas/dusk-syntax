use crate::term;
use crate::code_reference::{CodeLines, CharRef};

/// The macro, that simplifies terminal color setting
macro_rules! set_col {
    ($term: ident, $color: ident) => {
        $term.fg(term::color::$color).unwrap();
    };
}

/// The macro, that simplifyes terminal attributes setting
macro_rules! set_attr {
    ($term: ident, $attr: ident) => {
        $term.attr(term::Attr::$attr).unwrap();
    };
}

/// The macro, that simplifies terminal reset process
macro_rules! set_blank {
    ($term: ident) => {
        $term.reset().unwrap();
        print!("");
    };
}

/// The macro, that wrapps up every step of colored output
macro_rules! print_col {
    ($term: ident, $color:ident, $($printed:expr), *) => {
        set_col!($term, $color);
        print!($(
                $printed,
                )*);
        set_blank!($term);
    };
}

/// The macro, that wrapps up every step of colored output with additional
/// attributes, e.g. Bold
macro_rules! print_attr {
    ($term: ident, $color:ident, $attr:ident, $($printed:expr), *) => {
        set_attr!($term, $attr);
        print_col!($term, $color $(
                , $printed
                )*);
    };
}

/// Function that prints out an error message
///
/// This function takes as input the initial code, in which the error occured,
/// the starting and ending character references to point to, error code,
/// the error name and a help message.
#[allow(unused_parens)]
pub fn error (
    element: crate::ElementReference,
    error_code: &str,
    error_explained: &str,
    help_message: &str
    ) {

    ///////////////////////////////////////////////////////////////////////////
    //***********************COMMON THINGS FIRST*****************************//
    ///////////////////////////////////////////////////////////////////////////

    // Get links to needed character references and to the code
    let file: std::rc::Rc<CodeLines> = element.first.origin.clone();
    let start: CharRef = element.first.clone();
    let end: CharRef = element.last.clone();

    // FIRST LINE contains error code and its explained meaning
    let mut term = term::stdout().unwrap();
    print_attr!(term, BRIGHT_RED, Bold, "error[{}]", error_code);
    print_attr!(term, WHITE, Bold, ": {}\n", error_explained);

    // SECOND LINE contains the file, line and position, the error was found in
    let size: usize = format!("{}", end.line + 1).len();
    print_attr!(term, BRIGHT_BLUE, Bold, "{}--> ", " ".repeat(size));
    println!("{}:{}:{}", file.name, start.line + 1, start.pos + 1);

    // THIRD LINE is empty
    print!("{} | \n", " ".repeat(size));

    ///////////////////////////////////////////////////////////////////////////
    //***********************THEN IF ONLY ONE LINE***************************//
    ///////////////////////////////////////////////////////////////////////////
    if (start.line == end.line) {
        // If the error started and ended on the same line, indicating it
        // is pretty easy

        // The fourth line contains the line where error occured
        print_attr!(term, BRIGHT_BLUE, Bold, "{} | ", start.line + 1);
        for symbol in &file.code[start.line] {
            print!("{}", symbol);
        }

        // The fifth line indicates where exactly in this line it occured
        // and what it was
        print_attr!(term, BRIGHT_BLUE, Bold, "\n{} | ", " ".repeat(size));
        // First get the spaces right to get to the spot where error
        // starts (\t counts like one symbol but is 8 spaces wide)
        for symbol_number in 0..start.pos {
            if file.code[start.line][symbol_number] == '\t' {
                print!("        ");
                continue;
            }
            print!(" ");
        }
        // Then underline it with ^ signes
        for symbol_number in start.pos..end.pos {
            if file.code[start.line][symbol_number] == '\t' {
                print_attr!(term, BRIGHT_RED, Bold, "^^^^^^^^");
                continue;
            }
            print_attr!(term, BRIGHT_RED, Bold, "^");
        }
        // And output the help message
        print_attr!(term, BRIGHT_RED, Bold, "^{}\n", help_message);
    } else {
        ///////////////////////////////////////////////////////////////////////
        //***********************THEN IF MORE THAN ONE LINE******************//
        ///////////////////////////////////////////////////////////////////////
        // If the error contains multiple lines, it becomes harder

        for line_number in start.line..(end.line + 1) {
            // First check if the line should be normally rendered
            if  ( (line_number - start.line < 4) ||
                  (end.line - line_number < 2) ||
                  ( (line_number - start.line == 4) &&
                    (end.line-line_number == 2) ) ) {

                // If it should, then
                // Print the line number
                print_attr!(term, BRIGHT_BLUE, Bold, "{}", line_number + 1);
                // If it's too short, pad it with spaces
                print!("{}", " ".repeat(size - format!("{}", line_number + 1).len()));
                // Print the delimiter
                print_attr!(term, BRIGHT_BLUE, Bold, " | ");
                // If it's the first line, print a slash, otherwize a pipe
                if line_number == start.line {
                    print_attr!(term, BRIGHT_RED, Bold, "/ ");
                } else {
                    print_attr!(term, BRIGHT_RED, Bold, "| ");
                }
                // And finally print the line itself
                for symbol in &file.code[line_number] {
                    print!("{}", symbol);
                }
                println!();
            }
            ///////////////////////////////////////////////////////////////////
            //*******************IF THERE ARE TOO MANY LINES*****************//
            ///////////////////////////////////////////////////////////////////

            // If it's the fifth line, but there are more then 2 left until
            // the end, then show ...
            if ( (line_number - start.line == 4) &&
                 (end.line - line_number > 2) ) {

                // First print the ... indicator, then if needed pad it with
                // spaces to match the needed size + 3 length
                print_attr!(term, BRIGHT_BLUE, Bold, "...{}", " ".repeat(size));
                // Then print the red pipe
                print_attr!(term, BRIGHT_RED, Bold, "| ");
                println!();
            }
        }
        ///////////////////////////////////////////////////////////////////////
        //***********************DRAW THE ARROW******************************//
        ///////////////////////////////////////////////////////////////////////
        // And finish pointing to the end of the error, while mentioning
        // what it was
        print_attr!(term, BRIGHT_BLUE, Bold, "{} | ", " ".repeat(size));
        print_attr!(term, BRIGHT_RED, Bold, "|");
        // Draw the arrow
        for symbol_number in 0..end.pos {
            if file.code[end.line][symbol_number] == '\t' {
                print_attr!(term, BRIGHT_RED, Bold, "________");
                continue;
            }
            print_attr!(term, BRIGHT_RED, Bold, "_");
        }
        // And print the help message
        print_attr!(term, BRIGHT_RED, Bold, "^{}\n", help_message);
    }
    println!();
}

/// Function that prints a debug point message
///
/// This function is pretty much the same as [`error`], but much simpler
pub fn debug_at (character: &CharRef, message: &str) {
    let mut term = term::stdout().unwrap();

    // Print info about where is the symbol located
    print_attr!(
        term,
        GREEN,
        Bold,
        "Debug message at character {}:{} in file {} \n",
        character.line + 1,
        character.pos + 1,
        character.origin.name
        );

    // Then print the line it was found in
    for symbol_number in 0..character.pos {
        print!("{}", character.origin.code[character.line][symbol_number]);
    }
    // While highlighting the symbol itself
    print_attr!(term, GREEN, Bold, "{}", character.value);
    for symbol_number in (character.pos + 1)..character.origin.code[character.line].len() {
        print!("{}", character.origin.code[character.line][symbol_number]);
    }
    print_attr!(term, GREEN, Bold, "{}\n", message);
}

pub fn exit (code: &CodeLines, error_msg: &str) {
    let mut term = term::stdout().unwrap();
    print_attr!(term, RED, Bold, "Unable to compile {}: {}\nExiting\n", code.name, error_msg);
    std::process::exit(1);
}
