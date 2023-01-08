use yew::prelude::*;

#[function_component(Producer)]
fn producer() -> Html {
    html!(
        <div>
            <h3>{"Producer"}</h3>
        </div>
    )
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html!(
        <div>
            <h3>{"Consumer"}</h3>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <div>
            <Producer/>
            <Consumer/>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
