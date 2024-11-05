pub enum MainMenu {
    Add,
    View,
    Remove,
    Update,
}

impl MainMenu {
    
    pub fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::Add),
            "2" => Some(MainMenu::View),
            "3" => Some(MainMenu::Remove),
            "4" => Some(MainMenu::Update),
            _ => None,
        }
    }

    pub fn show() {
        println!("");
        println!("== BILL MANAGER ==");
        println!("1. Add bill");
        println!("2. View bill"); 
        println!("3. Remove bill"); 
        println!("4. Update bill"); 
        println!("");
        println!("Enter selection: ");
    }
}