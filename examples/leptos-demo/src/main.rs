use charming::{
    component::{Axis, Title},
    element::AxisType,
    series::Line,
    Chart, WasmRenderer,
};
use leptos::*;

#[component]
fn App() -> impl IntoView {
    let action = create_action(|_input: &()| async {
        let chart = Chart::new()
            .title(Title::new().text("Demo: Yew + Charming"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        let renderer = WasmRenderer::new(600, 400);
        renderer.render("chart",&chart).unwrap();
    });

    on_cleanup(|| {
        if let Ok(instance) = WasmRenderer::instance_by_id("chart") {
            WasmRenderer::dispose(&instance);
        }
    });

    view! {
        <div>
            <button on:click=move |_| {action.dispatch(());}>"Show chart"</button>
            <div id="chart"></div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
