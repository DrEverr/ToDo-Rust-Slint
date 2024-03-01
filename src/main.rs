slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    use slint::Model;
    let ui = AppWindow::new()?;

    ui.on_add_todo({
        let ui_handle = ui.as_weak();
        move |text| {
            let ui = ui_handle.unwrap();
            let mut todos: Vec<ToDo> = ui.get_todos().iter().collect();
            todos.push(ToDo {
                text: text.to_string().into(),
                done: false,
            });

            let todos = std::rc::Rc::new(slint::VecModel::from(todos));
            ui.set_todos(todos.into());
        }
    });

    ui.on_delete_done_todos({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let todos: Vec<ToDo> = ui.get_todos().iter().filter(|todo| !todo.done).collect();
            let todos = std::rc::Rc::new(slint::VecModel::from(todos));
            ui.set_todos(todos.into());
        }
    });

    ui.run()
}
