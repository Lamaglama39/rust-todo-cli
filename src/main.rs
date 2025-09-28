use todo_cli::Todo;
use rusqlite::Result;

fn main() -> Result<()> {
    let action = std::env::args().nth(1).expect("Please specify an action");

    let todo = Todo::new()?;

    if action == "add" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        println!("{:?}, {:?}", action, item);
        todo.insert(item)?;
        println!("todo saved");
    } else if action == "complete" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        println!("{:?}, {:?}", action, item);
        if todo.complete(&item)? {
            println!("todo saved");
        } else {
            println!("'{}' is not present in the list or already completed", item);
        }
    } else if action == "list" {
        println!("Todo List:");
        todo.print_list()?;
    } else if action == "delete" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        println!("{:?}, {:?}", action, item);
        if todo.delete(&item)? {
            println!("todo deleted");
        } else {
            println!("'{}' is not present in the list", item);
        }
    }

    Ok(())
}
