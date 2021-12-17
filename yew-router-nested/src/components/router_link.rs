//! A component wrapping an `<a>` tag that changes the route.
use crate::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
    Switch,
};
use std::marker::PhantomData;
use yew::prelude::*;

use super::{Msg, Props};
use crate::RouterState;
use yew::virtual_dom::VNode;

/// An anchor tag Component that when clicked, will navigate to the provided route.
///
/// Alias to RouterAnchor.
#[deprecated(note = "Has been renamed to RouterAnchor")]
pub type RouterLink<T> = RouterAnchor<T>;

/// An anchor tag Component that when clicked, will navigate to the provided route.
#[derive(Debug)]
pub struct RouterAnchor<SW: Switch + PartialEq + Clone + 'static, STATE: RouterState = ()> {
    router: RouteAgentDispatcher<STATE>,
    _marker: PhantomData<SW>,
}

impl<SW: Switch + PartialEq + Clone + 'static, STATE: RouterState> Component
    for RouterAnchor<SW, STATE>
{
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(_: &Context<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        RouterAnchor {
            _marker: Default::default(),
            router,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                let route = Route::from(ctx.props().route.clone());
                self.router.send(RouteRequest::ChangeRoute(route));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        #[cfg(feature = "std_web")]
        use stdweb::web::event::IEvent;

        let route: Route<STATE> = Route::from(ctx.props().route.clone());

        #[cfg(feature = "std_web")]
        let cb = ctx.link().callback(|event: ClickEvent| {
            event.prevent_default();
            Msg::Clicked
        });
        #[cfg(feature = "web_sys")]
        let cb = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Clicked
        });

        html! {
            <a
                class={ctx.props().classes.clone()}
                onclick={cb}
                disabled={ctx.props().disabled}
                href={route.route}
            >
                {
                    #[allow(deprecated)]
                    &ctx.props().text
                }
                {ctx.props().children.iter().collect::<VNode>()}
            </a>
        }
    }
}
