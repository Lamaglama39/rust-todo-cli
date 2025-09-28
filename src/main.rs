use rusqlite::{Connection, Result, params};

#[derive(Debug)]
struct TodoItem {
    id: i32,
    task: String,
    completed: bool,
}

struct Todo {
    conn: Connection,
}

impl Todo {
    fn new() -> Result<Todo> {
        let conn = Connection::open("todo.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE
            )",
            [],
        )?;

        Ok(Todo { conn })
    }

    fn insert(&self, task: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO todos (task, completed) VALUES (?1, ?2)",
            params![task, false],
        )?;
        Ok(())
    }

    fn complete(&self, task: &str) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "UPDATE todos SET completed = TRUE WHERE task = ?1 AND completed = FALSE",
            params![task],
        )?;
        Ok(rows_affected > 0)
    }

    fn delete(&self, task: &str) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "DELETE FROM todos WHERE task = ?1",
            params![task],
        )?;
        Ok(rows_affected > 0)
    }

    fn list(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, task, completed FROM todos ORDER BY id")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                task: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        for todo in todo_iter {
            let todo = todo?;
            let status_text = if todo.completed { "[x]" } else { "[ ]" };
            println!("{} {}", status_text, todo.task);
        }
        Ok(())
    }
}

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
        todo.list()?;
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
