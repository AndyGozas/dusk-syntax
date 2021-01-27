/// A structure, containing a character and it's initial position in the
/// code as well as the code for the reference
///
/// This structure contains not only a character, but the reference counting
/// pointer to the initial code as CodeLines, as well as the character's
/// line number and it's position in it. This is used mostly for debug
/// messages, but also for storing the precompiled strings extracted from
/// the code in \0 characters. If a \0 character is found in the code, it
/// means that initially there was a static string in it's place, which now
/// is placed in a vector of all strings, under the number, contained in
/// the line field.
#[derive(Clone)]
pub struct CharRef {

    /// This field contains a single character, extracted from the initial
    /// code
    pub value: char,

    /// This field contains a single unsigned integer value, representing
    /// the line, where the character came from
    ///
    /// If the character is \0, this field starts representing the position of
    /// a corresponding static string in the vector of static strings
    pub line: usize,

    /// This field contains a single unsigned integer value, representing
    /// the position of the character in the line, where it came from
    pub pos: usize,

    /// The reference pointing to the CodeLines object, that exact
    /// CharRef was created from
    pub origin: std::rc::Rc<CodeLines>,
}

impl CharRef {

    /// This function creates a character reference from a character and
    /// it's position
    ///
    /// This function is actually mainly used for internal purposes,
    /// as the primary way of turning the code into a vector of char
    /// references is by using [`CodeLines::get_char_ref`] function
    /// on the code, stored as a CodeLines object
    pub fn new (value: char,
                line: usize,
                pos: usize,
                origin: std::rc::Rc<CodeLines>
                ) -> CharRef {
        CharRef { value: value, line: line, pos: pos, origin: origin }
    }
}

/// A structure, containing the name of the file, the code came from and the
/// code itself as a vector of strings, represented by vectors of characters
/// in them
pub struct CodeLines {

    /// A vector of vectors of characters, representing lines of code
    /// read from the file
    pub code: Vec<Vec<char>>,

    /// The file name, where the code came from
    pub name: String,
}

impl CodeLines {

    /// A function that builds a code reference from the file name
    /// and it's contents
    ///
    /// # Example
    /// ```
    /// let if_main_cont: std::io::Result<String> =
    ///     fs::read_to_string(main_file);
    /// if if_main_cont.is_err() {
    ///     std::process::exit(1);
    /// }
    /// let main_cont: String = if_main_cont.unwrap();
    /// code_lines = CodeLines::new(main_file, main_cont);
    /// ```
    pub fn new (name: &String, input_text: &String) -> CodeLines {
        let mut tmp_string: Vec<char> = Vec::new();
        let mut result: CodeLines = CodeLines{
            name: name.to_string(),
            code: Vec::new(),
        };
        for character in input_text.chars() {
            tmp_string.push(character);
            if character == '\n' {
                result.code.push(tmp_string);
                tmp_string = Vec::new();
            }
        }
        result
    }

    /// A function that creates a vector of character references representing
    /// the code, stored in the CodeLines structure provided
    ///
    /// # Example
    /// ```
    /// let input_code: Vec<CharRef> = code_lines.get_char_ref();
    /// ```
    pub fn to_char_ref (origin: std::rc::Rc<CodeLines>) -> Vec<CharRef> {
        let mut result: Vec<CharRef> = Vec::new();
        for line_number in 0..origin.code.len() {
            for char_number in 0..origin.code[line_number].len() {
                result.push(CharRef::new(origin.code[line_number][char_number],
                                         line_number,
                                         char_number,
                                         origin.clone(),
                                         ));
            }
        }
        result
    }
}
