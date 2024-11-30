use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or_default]
    pub message: Option<String>,
}

pub struct Loading;

impl Component for Loading {
    type Message = ();
    type Properties = LoadingProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="loading-container" role="status" aria-busy="true">
                <div class="loading-spinner"></div>
                if let Some(message) = &ctx.props().message {
                    <p class="loading-message">{message}</p>
                }
            </div>
        }
    }
} 
