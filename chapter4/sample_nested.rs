//-- #########################
//-- Task: To create a sample nested_mod module

mod sample_mod {
    pub mod nested_mod {
        pub fn function() {
            println!("called `sample_mod::nested_mod::function()` \n");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `sample_mod::private_function()` \n");
        }
    }

    mod private_nested_mod {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `sample_mod::private_nested_mod::function()` \n");
        }
    }
}

fn main() {
    sample_mod::nested_mod::function();
}
