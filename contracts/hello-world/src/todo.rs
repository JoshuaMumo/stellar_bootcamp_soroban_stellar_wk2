use soroban_sdk::{Env, String, Symbol, Vec, contract, contractimpl,contracttype, symbol_short};

#[contracttype]
#[derive(Debug, Clone, PartialEq,Eq)]
pub struct Todo{
    pub id: u32,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[contract]
pub struct TodoList;

const TODOS: Symbol = symbol_short!("TODOS");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contractimpl]
impl TodoList {
    pub fn create_todo(env: Env, title: String, description: String) -> Todo {

        let current_id: u32 = env.storage().temporary().get(&NEXT_ID).unwrap_or(1);

        let mut todos: Vec<Todo> = env
            .storage()
            .temporary()
            .get(&TODOS)
            .unwrap_or(Vec::new(&env));

        let new_todo = Todo {
            id: current_id,
            title,
            description,
            is_completed: false,
        };

        todos.push_back(new_todo.clone());

        env.storage().temporary().set(&TODOS, &todos);

        env.storage()
            .temporary()
            .set(&NEXT_ID, &(current_id + 1));

        new_todo
    }

    pub fn update_todo(env: &Env, id: u32, new_title: String, new_description: String) -> bool {
        let mut todos = Self::get_todos(env);

        for i in 0..todos.len(){
            if let Some(mut todo) = todos.get(i){
                if todo.id == id{
                    todo.title = new_title;
                    todo.description = new_description;
                    todo.is_completed = false;
                    todos.set(i,todo);
                    env.storage().temporary().set(&TODOS, &todos);
                    return true;
                }
            }
        }
        false
    }

    pub fn get_todos(env: &Env) -> Vec<Todo> {
        env.storage().temporary().get(&TODOS).unwrap_or(Vec::new(env))
    }

    pub fn get_next_id(env: Env) -> u32 {
        env.storage().temporary().get(&TODOS).unwrap_or(1)
    }

    pub fn mark_as_completed(env: &Env, id: u32) -> bool {
        // loads the full todo list
        let mut todos = Self::get_todos(&env);

        // loop to iterate through the id's to find the matching id 
        for i in 0..todos.len() {
            if let Some(mut todo) = todos.get(i) {
                // if id matches in the struct field is_completed is to be set as true
                if todo.id == id {
                    todo.is_completed = true;
                    todos.set(i, todo);
                    env.storage().temporary().set(&TODOS, &todos);
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_todo(env: &Env, id: u32) -> bool {

        let todos = Self::get_todos(&env);
        // initialize a new vector called updated 
        let mut updated: Vec<Todo> = Vec::new(&env);
        // if an id is not found return false
        let mut found = false;

        // initialize a for loop to iterate the id's to find the matching id
        for i in 0..todos.len() {
            // if the id is found return true
            if let Some(todo) = todos.get(i) {
                if todo.id == id {
                    found = true; 
                } else {
                    // all the other items are pushed to the new vector called updated
                    updated.push_back(todo);
                }
            }
        }
        // if the id is foung it is then pushed to the new vector called updated
        if found {
            env.storage().temporary().set(&TODOS, &updated);
        }

        found
    }
}

