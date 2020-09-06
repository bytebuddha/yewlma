use yew::prelude::*;
use yewlma::prelude::*;

use crate::components::DemoContainer;

pub struct NotificationPage(ComponentLink<Self>);

impl Component for NotificationPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NotificationPage(link)
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Colors"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Notification color=Color::Danger>
                      <div class="content">
                        <p>{"Notification content"}</p>
                      </div>
                    </Notification>
                    <Notification color=Color::Primary delete=false>
                      <div class="content">
                        <p>{"Notification content"}</p>
                      </div>
                    </Notification>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
            </Container>
        }
    }
}
