use codee::string::FromToStringCodec;
use gloo::{dialogs::alert, net::http::Request};
use leptos::*;
use leptos_use::UseWebSocketReturn;
use paekli_core::http_api::{ReceiveRequest, ReceiveResponse, SendRequest};

fn main() {
    console_error_panic_hook::set_once();

    let (person, set_person) = create_signal(String::default());
    let (content, set_content) = create_signal(String::default());

    let send_paekli = move |_| {
        let recipient = Some(person.get());
        let content = content.get();
        let request = Request::post("https://paekli.buenzli.dev/paekli")
            .json(&SendRequest {
                content,
                recipient,
                express: false,
            })
            .unwrap();
        spawn_local(async {
            request.send().await.unwrap();
        });
        set_person.set(String::default());
    };

    let receive_paekli = move |_| {
        let recipient = person.get();
        let request = Request::delete("https://paekli.buenzli.dev/paekli")
            .json(&ReceiveRequest { recipient })
            .unwrap();
        spawn_local(async {
            let resp = request
                .send()
                .await
                .unwrap()
                .json::<ReceiveResponse>()
                .await;
            match resp {
                Ok(ReceiveResponse { content }) => {
                    alert(&format!("here's your paekli:\n{}!", content))
                }
                Err(_) => alert("no paekli"),
            }
        });
    };

    let (should_listen_to_notifications, set_should_listen_to_notifications) = create_signal(false);
    let toggle_should_listen_to_notifications =
        move |_| set_should_listen_to_notifications.update(|prev| *prev = !*prev);

    mount_to_body(move || {
        view! {
            <h1>Hello WebAssembly!</h1>
            "Mailbox: "
            <input
                placeholder="name"
                prop:value=move || person.get()
                on:input=move |e| set_person.set(event_target_value(&e))
            ></input>
            <br/>
            "Content: "
            <input
                placeholder="content"
                prop:value=move || content.get()
                on:input=move |e| set_content.set(event_target_value(&e))
            ></input>
            <br/>
            <button on:click=send_paekli>
                Send
            </button>
            <button on:click=receive_paekli>
                Receive
            </button>
            <button on:click=toggle_should_listen_to_notifications>
                enable websocket notifications
            </button>
            <Show when=move || should_listen_to_notifications.get()>
                <NotificationListener recipient=person.get() />
            </Show>
        }
    })
}

#[component]
fn NotificationListener(recipient: String) -> impl IntoView {
    let url = format!("ws://paekli.buenzli.dev/notifications/{recipient}");
    let UseWebSocketReturn { message, .. } =
        leptos_use::use_websocket::<String, String, FromToStringCodec>(&url);

    create_effect(move |_| {
        if let Some(message) = message.get() {
            gloo::dialogs::alert(&message);
        }
    });

    view! {
        <br/>
        "Listening to notifications for " {recipient} "..."
    }
}
