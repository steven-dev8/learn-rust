//! My Crate
//! 
//! "my_crate" é uma crate com ferraments úteis para desenvolvimento
//! matemático, ótimo para resoluções simples e que não envolva
//! calcúlos complexos

/// Adiciona um ao número de 32bits passado como argumento
///
/// # Exemplos
///
///```
/// let arg = 5;
/// let resposta = my_crate::soma_um(arg);
/// 
/// assert_eq!(6, resposta);
///```
pub fn soma_um(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;
}
