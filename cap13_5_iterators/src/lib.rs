#[derive(Debug, PartialEq)]
pub struct Shoe {
    size: u32,
    brand: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 30,
                brand: String::from("Nike"),
            },
            Shoe {
                size: 36,
                brand: String::from("Nike"),
            },
            Shoe {
                size: 44,
                brand: String::from("Nike"),
            },
            Shoe {
                size: 31,
                brand: String::from("Nike"),
            },
        ];

        let filter_shoes = shoes_in_size(shoes, 30);
        assert_eq!(
            filter_shoes,
            vec![
                Shoe {
                    size: 30,
                    brand: String::from("Nike"),
                }
            ],
        )
    }
}
