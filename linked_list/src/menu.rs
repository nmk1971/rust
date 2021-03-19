use std::io;
struct Node{
    value:i32,
    next:Option<Box<Node>>
}
pub fn display_menu(){
    let List:Option<Node> = None;
    let menu_items=[
        "1- Afficher",
        "2- Ajouter",
        "3- Supprimer",
        "4- Insérer",
        "5- Quitter",
    ];
    //println!("{}", menu_items.len());
    loop {
        println!("*****************************************");
        for mi in menu_items.iter(){ //Afficher les éléments du menu
            println!("{}", mi);
        }
        println!("Choisir : ");
        let choice:usize;
        loop{ //Choix
            let mut input_text = String::new();
            io::stdin()
                  .read_line(&mut input_text)
                  .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            
            match trimmed.parse::<usize>() {
                Ok(i) => 
                    
                        if i >=1 && i<=menu_items.len() {choice = i;break},
                    
                Err(..) => println!("this was not an integer: {}", trimmed),
            };
            
        }
        
        if choice==menu_items.len(){
                println!("Bye!!!!");
                break;
            }
    }
}