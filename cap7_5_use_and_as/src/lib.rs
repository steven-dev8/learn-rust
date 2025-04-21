use std::fmt::Result;
use std::io::Result as IoResult;
// use é usado para criar um atalho, já imaginou acessar algo sempre passando o caminho
// o código perderia legibilidade e ficaria muito repetitivo

use std::{cmp::Ordering, io};
// use std::cmp::Ordering
// use std::io;

use std::io::{self, Write};
// use std::io;
// use std::io::Write;

use std::collections::*;
// importa tudo

// não pode haver dois caminhos que importem com o mesmo nome
// std::io::Result - Isso daria um erro, foi possivel solucionar usando um aliase

// Segunda alternativa
// std::fmt;
// std::io

fn function1() -> Result {
 // --snip--
}

fn function2() -> IoResult() {
 // --snip--
}