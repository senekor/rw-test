use gloo::dialogs::alert;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    let (get_input, set_input) = create_signal(String::default());
    let (get_paekli, set_paekli) = create_signal(String::default());

    mount_to_body(move || {
        view! {
            <h1>Hello WebAssembly!</h1>
            <button
                on:click=move |_| {
                    set_paekli.set(get_input.get());
                    set_input.set(String::default());
                }
            >
                Send
            </button>
            <input
                placeholder="paekli content"
                prop:value=move || get_input.get()
                on:input=move |e| set_input.set(event_target_value(&e))
            ></input>
            <button
                on:click=move |_| {
                    let content = get_paekli.get();
                    set_paekli.set(String::default());
                    if content.is_empty() {
                        alert("no paekli");
                    } else {
                        alert(&format!("here's your paekli:\n{}!", content));
                    }
                }
            >
                Receive
            </button>
        }
    })
}
