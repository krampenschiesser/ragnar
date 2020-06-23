use crate::state::{Filter, Msg, State, Task};
use ragnar_html_markup::event::{InputEvent, KeyboardEvent};
use ragnar_lib::{AppComponent, AppContext, AppNode, Node};

#[derive(Component, Clone)]
pub struct App {}

impl AppComponent for App {
    type Msg = Msg;
    type State = State;

    fn render(&self, state: &Self::State, mut ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        app! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1> "todos" </h1>
                        <TaskInput />
                    </header>
                    <section class="main">
                        <InputCheckBox
                            class="toggle-all"
                            checked={state.is_all_completed()}
                            onclick={ctx.create_callback(|_| Msg::ToggleAll)} />
                        <ul class="todo-list">
                            { state.entries.iter().filter(|e| state.filter.fit(e)).enumerate().map(|(idx, e)| self.view_entry(&mut ctx, idx,e)).collect::<Vec<_>>() }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{ state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { [Filter::Active,Filter::All,Filter::Completed].iter().map(|flt| self.view_filter(flt,state)) }
                        </ul>
                        <button class="clear-completed" onclick={ctx.create_callback(|_| Msg::ClearCompleted)}>
                            { format!("Clear completed ({})", state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/krampenschiesser/" target="_blank">{ "Denis Kolodin and Christian Loehnert" }</a></p>
                    <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_entry(&self, ctx: &mut AppContext<Msg>, idx: usize, task: &Task) -> Node {
        let mut class = "todo".to_string();
        if task.editing {
            class.push_str(" editing");
        }
        if task.completed {
            class.push_str(" completed");
        }
        node! {
            <li class=class>
                <div class="view">
                    <InputCheckBox
                        class="toggle"
                        checked={task.completed}
                        onclick={ctx.create_callback(move |_| Msg::Toggle(idx))} />
                    <label ondblclick={ctx.create_callback(move |_| Msg::ToggleEdit(idx))}>
                        { task.description.clone() }
                    </label>
                    <button class="destroy" onclick={ctx.create_callback(move |_| Msg::Remove(idx))} />
                </div>
                { self.view_entry_edit_input(ctx, idx, &task) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, ctx: &mut AppContext<Msg>, idx: usize, task: &Task) -> Node {
        if task.editing {
            node! {
                <InputText class="edit"
                       value={task.description.clone()}
                       oninput={ctx.create_callback(|e: &InputEvent| Msg::UpdateEdit(e.data.clone()))}
                       onblur={ctx.create_callback(move |_| Msg::Edit(idx))}
                       onkeypress={ctx.create_callback(move |e: &KeyboardEvent| {
                          if e.key == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                       })} />
            }
        } else {
            node! { <InputHidden /> }
        }
    }

    pub fn view_filter(&self, filter: &Filter, state: &State) -> Node {
        node! {<FilterView filter={filter.clone()} />}
    }
}
