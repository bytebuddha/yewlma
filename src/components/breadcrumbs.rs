use yew::prelude::*;
use yewtil::NeqAssign;
use yew_route_breadcrumbs::Crumb;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest;

pub struct BreadCrumbs {
    link: ComponentLink<Self>,
    props: Props
}

pub struct Clicked(Option<&'static str>);

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub crumbs: Option<Vec<Crumb>>
}

impl Component for BreadCrumbs {
    type Message = Clicked;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Some(route) = msg.0 {
            let route = Route::new_no_state(route);
            RouteAgentDispatcher::new().send(RouteRequest::ChangeRoute(route));
        }
        false
    }

    fn view(&self) -> Html {
        if let Some(crumbs) = &self.props.crumbs {
            let length = crumbs.len() - 1;
            html! {
              <nav class="breadcrumb" aria-label="breadcrumbs">
                <ul>
                  {
                      for crumbs.iter().enumerate().map(|(dex, item)| {
                          let route = item.route.clone();
                          let callback = self.link.callback(move |_| Clicked(route));
                          let active = (dex == length).then_some("is-active");
                          html_nested!{
                              <li class=active><a onclick=callback>{item.text}</a></li>
                      }})
                  }
                </ul>
              </nav>
            }
        } else {
            html! {

            }
        }
    }
}
