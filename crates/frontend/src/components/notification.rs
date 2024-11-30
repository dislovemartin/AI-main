use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub message: String,
    pub notification_type: NotificationType,
    pub on_close: Callback<()>,
}

#[derive(PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    Info,
}

pub struct Notification;

impl Component for Notification {
    type Message = ();
    type Properties = NotificationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let class = match props.notification_type {
            NotificationType::Success => "notification success",
            NotificationType::Error => "notification error",
            NotificationType::Info => "notification info",
        };

        html! {
            <div
                class={class}
                role="alert"
                aria-live="polite"
            >
                <p>{&props.message}</p>
                <button
                    onclick={ctx.props().on_close.reform(|_| ())}
                    aria-label="Close notification"
                >
                    {"✕"}
                </button>
            </div>
        }
    }
}
