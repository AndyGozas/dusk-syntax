extern crate term;

use std::any::Any;

/// Module, that contains structures, needed for debug messages
pub mod code_reference;

pub mod warn;

pub struct ElementReference {
    pub first: code_reference::CharRef,
    pub last: code_reference::CharRef,
    pub element: Element,
}


/// The enum, that represents an element, extracted from the code,
/// and contains not only the element, but also some info, needed
/// for debug messages to be produced at compile time
pub enum Element {

    /// If the element, extracted from the code, is already just
    /// an object, which does not need any actions taken on it
    /// to be processed later on, e.g. a static string or a
    /// number, it should be put in the [`ElementReference::Object`]
    ///
    /// It contains a value inside of a [`Box<dyn Any>`], which can
    /// be downcasted to the type needed later on
    Object {
        value: Box<dyn Any>,
    },

    /// If the element is a function call, the function's name and
    /// its arguments should be placed in a
    /// [`ElementReference::Function`]
    ///
    /// Function name then will be stored as a String, and the
    /// arguments as a vector of Element References
    Function {
        name: String,
        args: Vec<ElementReference>,
    },
}
