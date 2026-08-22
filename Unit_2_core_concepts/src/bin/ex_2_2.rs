mod kitchen {
    fn secret_recipe() -> &'static str {
        "42 spices"
    }
    pub fn menu() -> &'static str {
        "Today's special"
    }

    pub mod staff {
        pub fn cook() -> String {
            format!("Cooking with {}", super::secret_recipe())
        }
    }
}

fn main() {
    println!("{}", kitchen::menu()); // Line A
    // println!("{}", kitchen::secret_recipe()); // Line B
    println!("{}", kitchen::staff::cook()); // Line C

    // Line A: ✅ Compiles — menu() is pub

    // Line B: ❌ Compile error — Line B (kitchen::secret_recipe()) fails to compile because of Rust’s strict privacy boundaries. In Rust, functions are private by default, meaning secret_recipe() is only accessible to items inside the kitchen module or its submodules. Since main() lives outside of kitchen, it acts as an "outsider" and is denied direct access.

    // Line C: ✅ Compiles — Line C (kitchen::staff::cook()) compiles successfully due to two distinct permission checks:
    //      Calling cook() from main(): Works because both the staff module (pub mod staff) and the cook() function (pub fn cook()) are marked with the pub keyword, making them accessible to main().

    //      Calling secret_recipe() inside cook(): Works because of Rust's parent-child access rules. The staff module is a child of kitchen. In Rust, child modules automatically have full access to everything in their parent modules, including private functions. The super:: keyword explicitly navigates up to the parent kitchen module to call secret_recipe().)
}
