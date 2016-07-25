// A compilation of notes and lessons from https://doc.rust-lang.org/book/comments.html

// Yes, comments requires its own file - it's a bit more complex than just //
// Not much more complex but still :)

// Regular comments are like the ones I've been doing. Duh.

// Another type of comment is a doc comment, which is differentiated via /// instead of //
// These types of comments support Markdown within them and are commonly used to provide documentation
// and usage instructions for methods

// Another type of doc comment is //!, which is used commonly in module or crate root files (lib.rs and mod.rs)
// Using //! vs /// seems to just be a style/convention thing

// Interestingly - you can use the rustdoc tool (https://doc.rust-lang.org/book/documentation.html) to create HTML
// documents from doc comments and run code examples (from within the comments) as tests
