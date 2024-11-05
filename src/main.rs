use billinglib::billing::BillCollection as bc;
use billinglib::billing_inputs::*;
use billinglib::main_menu::*;
use billinglib::menu_action;



fn run_program() -> Option<()> {
    let mut bill_collection: bc = bc::new();

    loop {
        MainMenu::show();
        let input: String = get_string()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::Add) => menu_action::add_bill(&mut bill_collection),
            Some(MainMenu::View) => menu_action::view_bills(&bill_collection),
            Some(MainMenu::Remove) => menu_action::remove_bill(&mut bill_collection),
            Some(MainMenu::Update) => menu_action::update_bill(&mut bill_collection),
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}
