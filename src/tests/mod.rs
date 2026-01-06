#[cfg(test)]
mod tests {
    use crate::TaskError;
    use crate::TaskList;

    #[test]
    fn empty_task_rejected_test() {
        let mut task_list: TaskList = TaskList::new();
        let result: Result<(), TaskError> = task_list.add_task("      ".to_string());
        assert!(result.is_err());
    }

    #[test]
    /// Kitchen sink test - add, list, etc.
    fn kitchen_sink_test() {
        let mut task_list = TaskList::new();
        let task1: &str = "Task 1";
        let task2: &str = "Task 2";
        let task3: &str = "Task 3";

        // add task 1
        let result: Result<(), TaskError> = task_list.add_task(task1.to_string());
        assert!(result.is_ok());
        assert_eq!(task_list.tasks.len(), 1);
        assert_eq!(task_list.tasks[0].description, task1);

        // add task 2
        let result: Result<(), TaskError> = task_list.add_task(task2.to_string());
        assert!(result.is_ok());
        assert_eq!(task_list.tasks.len(), 2);
        assert_eq!(task_list.tasks[1].description, task2);

        // add task 3
        let result: Result<(), TaskError> = task_list.add_task(task3.to_string());
        assert!(result.is_ok());
        assert_eq!(task_list.tasks.len(), 3);
        assert_eq!(task_list.tasks[2].description, task3);

        // remove task 2
        let result: Result<(), TaskError> = task_list.remove_task(2);
        assert!(result.is_ok());
        assert_eq!(task_list.tasks.len(), 2);

        // mark task 3 as completed
        let result: Result<(), TaskError> = task_list.complete_task(3);
        assert!(result.is_ok());
        assert_eq!(task_list.tasks[1].completed, true);

        // mark a non-existent task as completed
        let result: Result<(), TaskError> = task_list.complete_task(100);
        assert!(result.is_err());
    }
}
