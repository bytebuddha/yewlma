use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;
use yew_router::Switch;

use crate::classes::{Color, CssRepr, Size};
use crate::utils::NullSwitchImplementor;

pub enum Msg {
    Clicked,
}

pub struct Button<SW: Switch + Clone + 'static = NullSwitchImplementor, STATE: RouterState = ()> {
    props: Props<SW>,
    router: RouteAgentDispatcher<STATE>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props<SW: Switch + Clone + 'static> {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or("button")]
    pub tag: &'static str,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub fg: Option<Color>,
    #[prop_or_default]
    pub light: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub inverted: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub onsignal: Callback<()>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub route: Option<SW>,
}

impl<SW: Switch + Clone + 'static, STATE: RouterState> Component for Button<SW, STATE> {
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        Button {
            props,
            link,
            router,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
                if let Some(route) = &self.props.route {
                    let route = Route::from(route.clone());
                    self.router.send(RouteRequest::ChangeRoute(route));
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let fg = self.props.fg.is();
        let size = self.props.size.is();
        let color = self.props.color.is();
        let light = self.props.light.then_some("is-light");
        let outlined = self.props.outlined.then_some("is-outlined");
        let inverted = self.props.inverted.then_some("is-inverted");
        let rounded = self.props.rounded.then_some("is-rounded");
        let loading = self.props.loading.then_some("is-loading");
        let fullwidth = self.props.fullwidth.then_some("is-fullwidth");
        html! {
            <@{self.props.tag}
                 id?=self.props.id.as_ref()
                 disabled=self.props.disabled
                 onclick=self.link.callback(|_| Msg::Clicked)
                 class=(
                     "button", color, size, fg,
                     light, outlined, inverted, fullwidth,
                     rounded, loading, &self.props.class
                 )>
                 { self.props.children.clone() }
            </@>
        }
    }
}
