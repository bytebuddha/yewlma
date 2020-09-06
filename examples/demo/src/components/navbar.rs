use yew::prelude::*;
use yewlma::prelude::*;

use crate::routes::AppRoutes;
use yewlma::components::navbar::*;

pub struct DemoNavBar;

impl Component for DemoNavBar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        DemoNavBar
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <NavBar shadow=true color=Color::Primary>
              <Brand<AppRoutes> route=AppRoutes::Index>
                <h1 class="navbar-item title has-text-white">{"Yewlma"}</h1>
              </Brand<AppRoutes>>

              <div id="demoNavBar" class="navbar-menu">
                <div class="navbar-end">
                <Item<AppRoutes> route=AppRoutes::Layout>
                  {"Layout"}
                </Item<AppRoutes>>
                <div class="navbar-item has-dropdown is-hoverable">
                  <Link>{"Form"}</Link>
                  <div class="navbar-dropdown">
                    <Item<AppRoutes> route=AppRoutes::Input>
                      {"Input"}
                    </Item<AppRoutes>>
                  </div>
                </div>
                <div class="navbar-item has-dropdown is-hoverable">
                  <Link>{"Components"}</Link>
                  <div class="navbar-dropdown">
                    <Item<AppRoutes> route=AppRoutes::DropDown>
                      {"Drop Down"}
                    </Item<AppRoutes>>
                    <Item<AppRoutes> route=AppRoutes::Toaster>
                      {"Toaster"}
                    </Item<AppRoutes>>
                  </div>
                </div>
                  <div class="navbar-item has-dropdown is-hoverable">
                    <Link>{"Elements"}</Link>
                    <div class="navbar-dropdown">
                      <Item<AppRoutes> route=AppRoutes::Buttons>
                        {"Buttons"}
                      </Item<AppRoutes>>
                      <Item<AppRoutes> route=AppRoutes::Notifications>
                        {"Notifications"}
                      </Item<AppRoutes>>
                    </div>
                  </div>
                </div>
              </div>
            </NavBar>
        }
    }
}
