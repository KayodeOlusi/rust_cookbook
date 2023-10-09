//-- #########################
//-- Task: To create a sample module to illustrate how to use a module in rust

mod sample_mod {
    // By default all the items in module have private visibility

    fn private_function() {
        println!("called `sample_mod::private_function()` \n");
    }

    pub fn sample_function() {
        println!("called `sample_mod::sample_function()` \n");
    }

    pub fn indirect_private_fn() {
        print!("called `sample_mod::indirect_access()` that \n");
        private_function();
    }
}

fn sample_function() {
    println!("Called the `sample_function()` which is not a part of mod `sample_mod` \n");
}

fn main() {
    // Calling the sample_function which is outside module sample_function();
    sample_function();

    // Calling the public visible sample_mod's sample_function
    sample_mod::sample_function();

    // Accessing the private function indirectly
    sample_mod::indirect_private_fn();
}
