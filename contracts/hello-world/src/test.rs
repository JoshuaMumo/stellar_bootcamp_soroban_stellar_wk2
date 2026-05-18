#![cfg(test)]

use crate::todo::{TodoList, TodoListClient};


use soroban_sdk::{ Env, String};

struct SetUpResult<'a>{
    env: Env,
    client: TodoListClient<'a>,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    let contract_id = env.register(TodoList, ());

    let client = TodoListClient::new(&env, &contract_id);

    return SetUpResult{
        env,
        client
    }
}

#[test]
fn test_create_todo() {

    let setup_result = setup() ;
    
    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);
    assert_eq!(
        todo_creation.title, title
    );
    assert_eq!(
        todo_creation.description, description
    );
}

#[test]
fn test_update_todo() {
    let setup_result = setup();

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    let new_title = String::from_str(&setup_result.env, "stellar impact bootcamp wk 2");

    let new_description = String::from_str(&setup_result.env, "soroban smartcontract wk 2");

    let id = todo_creation.id;

    let update_todo_call = setup_result.client.update_todo(&id, &new_title, &new_description);

    assert!(update_todo_call, "failed to update");

    let todos = setup_result.client.get_todos();

    let updated_todo = todos.get(0).unwrap();

    assert_eq!(updated_todo.title, new_title);

    assert_eq!(updated_todo.description, new_description);

    // for todo in todos {
    //     assert_eq!(todo.title, new_title);
    //     assert_eq!(todo.description, new_description);
    // }

}

#[test]
fn test_mark_as_completed() {

    let setup_result = setup();
    
    // create a new title and description 

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    let id = todo_creation.id;

    // use mark_as_completed() function to change the boolean to true

    let mark_as_complete_call = setup_result.client.mark_as_completed(&id);

    assert!(mark_as_complete_call, "failed to mark_as_completed");

    let todos = setup_result.client.get_todos();

    let updated_todo = todos.get(0).unwrap();

    assert_eq!(updated_todo.id, id);
    assert_eq!(updated_todo.title, title);
    assert_eq!(updated_todo.description, description);



}

#[test]
fn test_delete_todo(){
    let setup_result = setup();

      // create a new title and description 

    let title = String::from_str(&setup_result.env, "stellar impact bootcamp");

    let description = String::from_str(&setup_result.env, "soroban smartcontract");

    let todo_creation = setup_result.client.create_todo(&title, &description);

    let id = todo_creation.id;

    // get all todo list
    
    let todos = setup_result.client.get_todos();

    // delete the item with the id in todo_creation.id

    let delete_todo_call = setup_result.client.delete_todo(&id);

    assert!(delete_todo_call, "failed to delete");


    //get all todo list 

    let updated_todo = todos.get(0).unwrap();

    assert_eq!(updated_todo.id, id);
    assert_eq!(updated_todo.title, title);
    assert_eq!(updated_todo.description, description);

}