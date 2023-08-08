use std::io::{BufReader, Cursor};

use leptos::*;
use libbfi::prelude::*;

#[component]
fn UserInputArea(cx: Scope, inputwrite: RwSignal<String>) -> impl IntoView {
    view! {cx,
        <textarea
            name="Brainfuck Code Here"
            id="bf_area"
            cols="30"
            rows="10"
            class="text-black bg-white w-full border-s-4 border-l-rose-500 border-black border-4 break-words overflow-auto mb-3 rounded-xl p-2
            hover:scale-105 focus-within:scale-105 transition-all ease-in-out resize-none"
            on:input=move |ev| {
                inputwrite.set(event_target_value(&ev));
            }>
            {move || inputwrite.get()}
        </textarea>
    }
}

fn string_from_vec(vec: Vec<u8>) -> String {
    return String::from(std::str::from_utf8(vec.as_slice()).expect("msg"));
}

#[component]
fn OutputBrainfuckArea(cx: Scope, output: ReadSignal<Vec<u8>>) -> impl IntoView {
    view! { cx,
        <textarea
            disabled
            name="Output here"
            id="bf_area"
            cols="30"
            rows="10"
            class="text-white bg-black w-full border-s-4 border-l-rose-500 break-words overflow-auto mt-3 rounded-xl p-3 resize-y">
            {move || string_from_vec(output.get())}
        </textarea>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let mut main_program_memory = BrainfuckMemory::new();
    let user_input = create_rw_signal(cx, ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.".to_string());
    let (userread, userwrite) = create_signal(cx, "".to_string());
    let mut writestream: Vec<u8> = Vec::new();
    let memory = create_rw_signal(cx, Vec::new() as Vec<u8>);

    view! { cx,
        <main
              class="border-gray-200 border-4 rounded-2xl bg-gray-100 drop-shadow-xl my-5 m-auto"
            >
                <UserInputArea inputwrite=user_input/>
                <input type="text" class="bg-white border-gray-200 shadow-md rounded-md text-black p-3 m-3" placeholder="input here" on:input=move |ev| {
                    userwrite.set(event_target_value(&ev));
                }></input>
                <div class="flex items-center justify-center">
                    <button
                      class="bg-white text-black rounded-md p-3 mx-3 transition delay-75 ease-in-out hover:scale-110 hover:bg-cyan-100 duration-200 border-2 border-gray-200 hover:border-gray-700"
                      on:click=move |_| {
                        memory.update(|thing| thing.clear());
                        main_program_memory
                            .add_tokens(user_input.read_only().get().chars()).expect("FUCK failed")
                            .run_full_stack(&mut BufReader::new(Cursor::new(userread.get().into_bytes())), &mut Cursor::new(&mut writestream))
                            .clean_env();
                        memory.update(|thing| thing.append(&mut writestream));
                     }
                      >Run</button
                    >
                    <button
                      class="bg-white text-black tai rounded-md p-3 mx-3 transition delay-75 ease-in-out hover:scale-110 hover:bg-rose-100 duration-200 border-2 border-gray-200 hover:border-gray-500"
                      >Stop</button
                    >
                </div>
                <OutputBrainfuckArea output=memory.read_only()/>
        </main>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,  <App/> }
    })
}
