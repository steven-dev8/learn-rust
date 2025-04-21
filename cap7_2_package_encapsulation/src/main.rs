use restaurant::front_of_house::hosting;


fn main() {
    hosting::add_to_waitlist();
}

// STRUCT
//crate
// └── front_of_house
//     ├── hosting
//     │   ├── add_to_waitlist
//     │   └── seat_at_table
//     └── serving
//         ├── take_order
//         ├── serve_order
//         └── take_payment