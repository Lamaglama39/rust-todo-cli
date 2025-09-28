use todo_cli::Todo;

#[test]
fn test_insert_task() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Insert a task
    todo.insert("Test task".to_string()).expect("Failed to insert task");

    // Check if the task was inserted
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].task, "Test task");
    assert_eq!(todos[0].completed, false);
}

#[test]
fn test_insert_multiple_tasks() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Insert multiple tasks
    todo.insert("Task 1".to_string()).expect("Failed to insert task 1");
    todo.insert("Task 2".to_string()).expect("Failed to insert task 2");
    todo.insert("Task 3".to_string()).expect("Failed to insert task 3");

    // Check if all tasks were inserted
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].task, "Task 1");
    assert_eq!(todos[1].task, "Task 2");
    assert_eq!(todos[2].task, "Task 3");
}

#[test]
fn test_complete_task() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Insert a task
    todo.insert("Complete me".to_string()).expect("Failed to insert task");

    // Complete the task
    let result = todo.complete("Complete me").expect("Failed to complete task");
    assert!(result, "Task should have been marked as completed");

    // Check if the task was completed
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].task, "Complete me");
    assert_eq!(todos[0].completed, true);
}

#[test]
fn test_complete_nonexistent_task() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Try to complete a task that doesn't exist
    let result = todo.complete("Nonexistent task").expect("Failed to complete task");
    assert!(!result, "Nonexistent task should not be marked as completed");
}

#[test]
fn test_complete_already_completed_task() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Insert and complete a task
    todo.insert("Already done".to_string()).expect("Failed to insert task");
    todo.complete("Already done").expect("Failed to complete task");

    // Try to complete the same task again
    let result = todo.complete("Already done").expect("Failed to complete task");
    assert!(!result, "Already completed task should not be marked as completed again");
}

#[test]
fn test_delete_task() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Insert a task
    todo.insert("Delete me".to_string()).expect("Failed to insert task");

    // Delete the task
    let result = todo.delete("Delete me").expect("Failed to delete task");
    assert!(result, "Task should have been deleted");

    // Check if the task was deleted
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 0);
}

#[test]
fn test_delete_nonexistent_task() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Try to delete a task that doesn't exist
    let result = todo.delete("Nonexistent task").expect("Failed to delete task");
    assert!(!result, "Nonexistent task should not be deleted");
}

#[test]
fn test_list_empty() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // List should be empty initially
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 0);
}

#[test]
fn test_list_with_mixed_status() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Insert tasks with different statuses
    todo.insert("Task 1".to_string()).expect("Failed to insert task 1");
    todo.insert("Task 2".to_string()).expect("Failed to insert task 2");
    todo.insert("Task 3".to_string()).expect("Failed to insert task 3");

    // Complete some tasks
    todo.complete("Task 2").expect("Failed to complete task 2");

    // Check the list
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].completed, false); // Task 1
    assert_eq!(todos[1].completed, true);  // Task 2
    assert_eq!(todos[2].completed, false); // Task 3
}

#[test]
fn test_workflow() {
    let todo = Todo::new_in_memory().expect("Failed to create in-memory todo");

    // Add some tasks
    todo.insert("Buy groceries".to_string()).expect("Failed to insert task");
    todo.insert("Do laundry".to_string()).expect("Failed to insert task");
    todo.insert("Study Rust".to_string()).expect("Failed to insert task");

    // Complete one task
    todo.complete("Buy groceries").expect("Failed to complete task");

    // Delete one task
    todo.delete("Do laundry").expect("Failed to delete task");

    // Check final state
    let todos = todo.list().expect("Failed to list todos");
    assert_eq!(todos.len(), 2);

    // Find the remaining tasks
    let grocery_task = todos.iter().find(|t| t.task == "Buy groceries");
    let rust_task = todos.iter().find(|t| t.task == "Study Rust");

    assert!(grocery_task.is_some());
    assert!(rust_task.is_some());
    assert!(grocery_task.unwrap().completed);
    assert!(!rust_task.unwrap().completed);
}