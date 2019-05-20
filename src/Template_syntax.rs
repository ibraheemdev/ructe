// This module is only a chapter of the documentation.
//! This module describes the template syntax used by ructe.
//!
//! The syntax is inspired by
//! [Twirl](https://github.com/playframework/twirl), the Scala-based
//! template engine in
//! [Play framework](https://www.playframework.com/),
//! but of course with rust types expressions instead of scala.
//!
//! A template consists of three basic parts:
//! First a preamble of `use` statements, each prepended by an @ sign.
//! Secondly a declaration of the parameters the template takes.
//! And third, the template body.
//!
//! ```html
//! @(name: &str, value: &u32)
//!
//! <html>
//!   <head><title>@name</title></head>
//!   <body>
//!     <p>The value of @name is @value.</p>
//!   <body>
//! </html>
//! ```
//!
//! As seen above, string slices and integers can easily be outputed
//! in the template body, using `@name` where `name` is a parameter of
//! the template.
//! Actually, more complex expressions can be outputed in the same
//! way, as long as the resulting value implements [`ToHtml`].
//! Rust types that implements [`Display`] automatically implements
//! [`ToHtml`] in such a way that contents are safely escaped for
//! html.
//!
//! ```html
//! @use any::rust::Type;
//!
//! @(name: &str, items: &[Type])
//!
//! <html>
//!   <head><title>@name</title></head>
//!   <body>
//!     @if items.is_empty() {
//!       <p>There are no items.</p>
//!     } else {
//!       <p>There are @items.len() items.</p>
//!       <ul>
//!       @for item in items {
//!         <li>@item</li>
//!       }
//!       </ul>
//!   <body>
//! </html>
//! ```
//!
//! The curly brackets, `{` and `}`, is used for blocks (see Loops,
//! Conditionals, and Calling other templates below).

//! To use verbatim curly brackets in the template body, they must be
//! escaped as `@{` and `@}`, the same goes for the `@` sign, that
//! precedes expressions and special blocks; verbtim `@` signs must be
//! escaped as `@@`.
//!
//! [`ToHtml`]: ../templates/trait.ToHtml.html
//! [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
#![allow(non_snake_case)]

pub mod a_Value_expressions {
    //! A value expression can be as simple as `@name` to get the value of
    //! a parameter, but more complicated expressions, including function
    //! calls, are also allowed.
    //!
    //! # Value expressions
    //!
    //! A parameter can be used in an expression preceded by an @ sign.
    //!
    //! ```text
    //! <h1>@name</h1>
    //! ```
    //!
    //! If a parameter is a struct or a trait object, its fields or methods can
    //! be used, and if it is a callable, it can be called.
    //!
    //! ```text
    //! <p>The user @user.name has email @user.get_email().</p>
    //! <p>A function result is @function(with, three, arguments).</p>
    //! ```
    //!
    //! Standard function and macros can also be used, e.g. for specific
    //! formatting needs:
    //!
    //! ```text
    //! <p>The value is @format!("{:.1}", float_value).</p>
    //! ```
    //!
    //! If more complex expressions are needed, they can be put in
    //! parenthesis.
    //!
    //! ```text
    //! <p>The sum @a+3 is @(a+3).</p>
    //! ```
    //!
    //! Anything is allowed in parenthesis, as long as parenthesis,
    //! brackets and string quotes are balanced.
    //! Note that this also applies to the parenthesis of a function
    //! call or the brackets of an index, so complex things like the
    //! following are allowed:
    //!
    //! ```text
    //! <p>Index: @myvec[t.map(|s| s.length()).unwrap_or(0)].</p>
    //! <p>Argument: @call(a + 3, |t| t.something()).</p>
    //! ```
}

pub mod b_Loops {
    //! A ructe `@for` loop works just as a rust `for` loop,
    //! iterating over anything that implements `std::iter::IntoIterator`,
    //! such as a `Vec` or a slice.
    //!
    //! # Loops
    //!
    //! Rust-like loops are supported like this:
    //!
    //! ```text
    //! <ul>@for item in items {
    //!   <li>@item</li>
    //! }</ul>
    //! ```
    //!
    //! Note that the thing to loop over (items, in the example) is a rust
    //! expression, while the contents of the block is template code.
    //!
    //! If items is a slice of tuples (or really, anything that is
    //! iterable yielding tuples), it is possible to deconstruct the
    //! tuples into separate values directly:
    //!
    //! ```text
    //! @for (n, item) in items.iter().enumerate() {
    //!     <p>@n: @item</p>
    //! }
    //! ```
    //!
    //! It is also possible to loop over a literal array (which may be
    //! an array of tuples), as long as you do it by reference:
    //!
    //! ```text
    //! @for &(name, age) in &[("Rasmus", 44), ("Mike", 36)] {
    //!     <p>@name is @age years old.</p>
    //! }
    //! ```
}

pub mod c_Conditionals {
    //! Both `@if` statements with boolean expressions and match-like
    //! guard `@if let` statements are supported.
    //!
    //! # Conditionals
    //!
    //! Rust-like conditionals are supported in a style similar to the loops:
    //!
    //! ```text
    //! @if items.is_empty() {
    //!   <p>There are no items.</p>
    //! }
    //! ```
    //!
    //! Pattern matching let expressions are also supported, as well as an
    //! optional else part.
    //!
    //! ```text
    //! @if let Some(foo) = foo {
    //!   <p>Foo is @foo.</p>
    //! } else {
    //!   <p>There is no foo.</p>
    //! }
    //! ```
    //!
    //! General rust `match` statements are _not_ supported in ructe
    //! (at least not yet).
}

pub mod d_Calling_other_templates {
    //! The ability to call other templates for from a template makes
    //! both "tag libraries" and "base templates" possible with the
    //! same syntax.
    //!
    //! # Calling other templates
    //!
    //! While rust methods can be called as a simple expression, there is a
    //! special syntax for calling other templates:
    //! `@:template_name(template_arguments)`.
    //! Also, before calling a template, it has to be imported by a `use`
    //! statement.
    //! Templates are declared in a `templates` module.
    //!
    //! So, given something like this in `header.rs.html`:
    //!
    //! ```text
    //! @(title: &str)
    //!
    //! <head>
    //!   <title>@title</title>
    //!   <link rel="stylesheet" href="/my/style.css" type="text/css">
    //! </head>
    //! ```
    //!
    //! It can be used like this:
    //!
    //! ```text
    //! @use super::header;
    //!
    //! @()
    //!
    //! <html>
    //!   @:header("Example")
    //!   <body>
    //!     <h1>Example</h1>
    //!     <p>page content ...</p>
    //!   </body>
    //! </html>
    //! ```
    //!
    //! It is also possible to send template blocks as parameters to templates.
    //! A structure similar to the above can be created by having something like
    //! this in `base_page.rs.html`:
    //!
    //! ```text
    //! @(title: &str, body: Content)
    //!
    //! <html>
    //!   <head>
    //!     <title>@title</title>
    //!     <link rel="stylesheet" href="/my/style.css" type="text/css">
    //!   </head>
    //!   <body>
    //!     <h1>@title</h1>
    //!     @:body()
    //!   </body>
    //! </html>
    //! ```
    //!
    //! And use it like this:
    //!
    //! ```text
    //! @use super::base_page;
    //!
    //! @()
    //!
    //! @:base_page("Example", {
    //!     <p>page content ...</p>
    //! })
    //! ```
}
