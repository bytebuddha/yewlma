use crate::components::DemoNavBar;
use crate::pages::*;
use crate::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewlma::prelude::*;
use yewlma::utils::{BulmaCdn, Cdn};
pub struct App(ComponentLink<Self>);

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App(link)
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <Cdn cdn=BulmaCdn::CdnJs />
            <DemoNavBar />
            <Toaster />
            <Container>
              <div class="box mt-2 has-background-grey-light">
                <Router<AppRoutes>
                    render = Router::render(|switch| {
                        match switch {
                            AppRoutes::Toaster => html! { <ToasterPage /> },
                            AppRoutes::Notifications => html! { <NotificationPage /> },
                            AppRoutes::Input => html! { <InputPage /> },
                            AppRoutes::Layout => html!{ <LayoutPage /> },
                            AppRoutes::Buttons => html! { <ButtonsPage />},
                            AppRoutes::DropDown => html! { <DropDownPage />},
                            AppRoutes::Index => html! { <IndexPage />}
                        }
                    })
                />
             </div>
            </Container>
            </div>
        }
    }
}
