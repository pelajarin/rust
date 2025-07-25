// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {
    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits::APPLE,
        delicious_snacks::veggies::CARROT,
    );
}
