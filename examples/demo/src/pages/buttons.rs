use yew::prelude::*;
use yewlma::prelude::*;

use crate::components::DemoContainer;
use crate::routes::AppRoutes;

pub struct ButtonsPage(ComponentLink<Self>);

impl Component for ButtonsPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonsPage(link)
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
                    <Buttons>
                      <Button color=Color::Primary>{"Primary"}</Button>
                      <Button color=Color::Link>{"Link"}</Button>
                      <Button color=Color::Info>{"Info"}</Button>
                      <Button color=Color::Danger>{"Danger"}</Button>
                      <Button color=Color::Warning>{"Warning"}</Button>
                      <Button color=Color::Success>{"Success"}</Button>
                      <Button color=Color::Dark>{"Dark"}</Button>
                      <Button color=Color::Light>{"Light"}</Button>
                      <Button color=Color::Black>{"Black"}</Button>
                      <Button color=Color::White>{"White"}</Button>
                    </Buttons>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Light"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Buttons>
                      <Button light=true color=Color::Primary>{"Primary"}</Button>
                      <Button light=true color=Color::Link>{"Link"}</Button>
                      <Button light=true color=Color::Info>{"Info"}</Button>
                      <Button light=true color=Color::Danger>{"Danger"}</Button>
                      <Button light=true color=Color::Warning>{"Warning"}</Button>
                      <Button light=true color=Color::Success>{"Success"}</Button>
                      <Button light=true color=Color::Dark>{"Dark"}</Button>
                      <Button light=true color=Color::Light>{"Light"}</Button>
                      <Button light=true color=Color::Black>{"Black"}</Button>
                      <Button light=true color=Color::White>{"White"}</Button>
                    </Buttons>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Outlined"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Buttons>
                      <Button outlined=true color=Color::Primary>{"Primary"}</Button>
                      <Button outlined=true color=Color::Link>{"Link"}</Button>
                      <Button outlined=true color=Color::Info>{"Info"}</Button>
                      <Button outlined=true color=Color::Danger>{"Danger"}</Button>
                      <Button outlined=true color=Color::Warning>{"Warning"}</Button>
                      <Button outlined=true color=Color::Success>{"Success"}</Button>
                      <Button outlined=true color=Color::Dark>{"Dark"}</Button>
                      <Button outlined=true color=Color::Light>{"Light"}</Button>
                      <Button outlined=true color=Color::Black>{"Black"}</Button>
                      <Button outlined=true color=Color::White>{"White"}</Button>
                    </Buttons>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Rounded"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Buttons>
                      <Button rounded=true color=Color::Primary>{"Primary"}</Button>
                      <Button rounded=true color=Color::Link>{"Link"}</Button>
                      <Button rounded=true color=Color::Info>{"Info"}</Button>
                      <Button rounded=true color=Color::Danger>{"Danger"}</Button>
                      <Button rounded=true color=Color::Warning>{"Warning"}</Button>
                      <Button rounded=true color=Color::Success>{"Success"}</Button>
                      <Button rounded=true color=Color::Dark>{"Dark"}</Button>
                      <Button rounded=true color=Color::Light>{"Light"}</Button>
                      <Button rounded=true color=Color::Black>{"Black"}</Button>
                      <Button rounded=true color=Color::White>{"White"}</Button>
                    </Buttons>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Disabled"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Buttons>
                      <Button disabled=true color=Color::Primary>{"Primary"}</Button>
                      <Button disabled=true color=Color::Link>{"Link"}</Button>
                      <Button disabled=true color=Color::Info>{"Info"}</Button>
                      <Button disabled=true color=Color::Danger>{"Danger"}</Button>
                      <Button disabled=true color=Color::Warning>{"Warning"}</Button>
                      <Button disabled=true color=Color::Success>{"Success"}</Button>
                      <Button disabled=true color=Color::Dark>{"Dark"}</Button>
                      <Button disabled=true color=Color::Light>{"Light"}</Button>
                      <Button disabled=true color=Color::Black>{"Black"}</Button>
                      <Button disabled=true color=Color::White>{"White"}</Button>
                    </Buttons>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Yew Router Support"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Button<AppRoutes> color=Color::Primary route=AppRoutes::Index>{"Go To Index"}</Button<AppRoutes>>
                  </DemoContainer>
                </Column>
              </Columns>
              </div>
            </Container>
        }
    }
}
