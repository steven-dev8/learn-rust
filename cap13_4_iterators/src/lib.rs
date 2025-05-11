pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut iter_v1 = v1.iter();
        // T.iter(); retorna um iterador com referências imutáveis Option<&T>
        
        assert_eq!(iter_v1.next(), Some(&1));
        assert_eq!(iter_v1.next(), Some(&2));
        assert_eq!(iter_v1.next(), Some(&3));
        assert_eq!(iter_v1.next(), None);

        let mut v2 = vec![1, 2, 3];

        let mut iter_v2 = v2.iter_mut();
        // T.iter_mut(); retorna um iterador com referências mutáveis Option<&mut T>
        // OBS: a coleção deve ser mutável, como no mut v2.
    
        assert_eq!(iter_v2.next(), Some(&mut 1));
        assert_eq!(iter_v2.next(), Some(&mut 2));
        assert_eq!(iter_v2.next(), Some(&mut 3));
        assert_eq!(iter_v2.next(), None);

        let v3 = vec!["Steven", "AMA", "Rosane"];
        let mut iter_v3 = v3.into_iter();
        // v3.into_iter(); retorna um iterador que move a coleção, no caso v3 aqui, então ao iterar
        // sobre os valores do iterador, ele estará consumindo também, e v3 perde a propriedade

        assert_eq!(iter_v3.next(), Some("Steven"));
        assert_eq!(iter_v3.next(), Some("AMA"));
        assert_eq!(iter_v3.next(), Some("Rosane"));
        assert_eq!(iter_v3.next(), None);

        // Métodos consumidores "proprietários"
        // .sum(), .collect(), .min(), .max()...
        // Adaptadores de iterador
        // .filter(), .map(), .zip()...
    }
}
