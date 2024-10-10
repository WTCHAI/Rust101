mod sausage_factory {
    pub fn get_secret_recipe() -> String {
        String::from("Secret is nothings!")
    }

    // Added `pub` before `fn` to make the function accessible outside the module.
    pub fn make_sausage() {
        println!("{}",String::from(get_secret_recipe()));
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
    let text : &str = &sausage_factory::get_secret_recipe() ; 
    println!("{}",&text) ; 
}
