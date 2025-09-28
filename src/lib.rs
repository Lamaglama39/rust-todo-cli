use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct TodoItem {
    pub id: i32,
    pub task: String,
    pub completed: bool,
}

pub struct Todo {
    conn: Connection,
}

impl Todo {
    pub fn new() -> Result<Todo> {
        let conn = Connection::open("todo.db")?;
        Todo::init_with_connection(conn)
    }

    pub fn new_in_memory() -> Result<Todo> {
        let conn = Connection::open_in_memory()?;
        Todo::init_with_connection(conn)
    }

    fn init_with_connection(conn: Connection) -> Result<Todo> {
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

    pub fn insert(&self, task: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO todos (task, completed) VALUES (?1, ?2)",
            params![task, false],
        )?;
        Ok(())
    }

    pub fn complete(&self, task: &str) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "UPDATE todos SET completed = TRUE WHERE task = ?1 AND completed = FALSE",
            params![task],
        )?;
        Ok(rows_affected > 0)
    }

    pub fn delete(&self, task: &str) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "DELETE FROM todos WHERE task = ?1",
            params![task],
        )?;
        Ok(rows_affected > 0)
    }

    pub fn list(&self) -> Result<Vec<TodoItem>> {
        let mut stmt = self.conn.prepare("SELECT id, task, completed FROM todos ORDER BY id")?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                task: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn print_list(&self) -> Result<()> {
        let todos = self.list()?;
        for todo in todos {
            let status_text = if todo.completed { "[x]" } else { "[ ]" };
            println!("{} {}", status_text, todo.task);
        }
        Ok(())
    }
}