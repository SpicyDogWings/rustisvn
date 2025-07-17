struct MenuItem {
    key: &'static str,
    label: &'static str
}

const MAIN_MENU: [MenuItem; 4] = [
    MenuItem { key: "s", label: "Status" },
    MenuItem { key: "f", label: "Filtrar"},
    MenuItem { key: "?", label: "Help" },
    MenuItem { key: "q", label: "Quit"}
];

pub fn print_main_menu() {
    println!("󰍜 Main Menu:\n");
    for item in MAIN_MENU.iter() {
        println!("{}. {}", item.key, item.label);
    }
    println!("\n");
}
