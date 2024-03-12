use gloo::dialogs::alert;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    let (get_content, set_content) = create_signal(String::default());

    mount_to_body(move || {
        view! {
            <h1>Hello WebAssembly!</h1>
            <button
                on:click=move |_| alert(&format!("paekli with {} was sent!", get_content.get()))
            >
                Send
            </button>
            <input
                placeholder="paekli content"
                prop:value=move || get_content.get()
                on:input=move |e| set_content.set(event_target_value(&e))
            ></input>
            <button
                on:click=move |_| {
                    let content = get_content.get();
                    set_content.set(String::default());
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
