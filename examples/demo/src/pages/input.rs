use yew::prelude::*;
use yewlma::prelude::*;

pub struct InputPage(ComponentLink<Self>);

impl Component for InputPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputPage(link)
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
                  <span class="title py-5 px-5">{"Input Types"}</span>
                </Columns>
                <Container>
                  <Columns>
                    <Column>
                      <InputField />
                    </Column>
                    <Column>
                      <InputField ty="password" />
                    </Column>
                    <Column>
                      <InputField ty="tel" />
                    </Column>
                  </Columns>
                </Container>
              </div>
              <div class="box has-background-color-grey-lighter">
                <Columns>
                  <span class="title py-5 px-5">{"Input Colors"}</span>
                </Columns>
                <Container>
                  <Columns>
                    <Column>
                      <InputField color=Color::Warning />
                    </Column>
                    <Column>
                      <InputField color=Color::Primary />
                    </Column>
                    <Column>
                      <InputField color=Color::Danger />
                    </Column>
                    <Column>
                      <InputField color=Color::Success />
                    </Column>
                    <Column>
                      <InputField color=Color::Info />
                    </Column>
                    <Column>
                      <InputField color=Color::Dark />
                    </Column>
                    <Column>
                      <InputField color=Color::Light />
                    </Column>
                    <Column>
                      <InputField color=Color::Black />
                    </Column>
                    <Column>
                      <InputField color=Color::White />
                    </Column>
                  </Columns>
                </Container>
              </div>
              <div class="box has-background-color-grey-lighter">
                <Columns>
                  <span class="title py-5 px-5">{"Loading"}</span>
                </Columns>
                <Container>
                  <Columns>
                    <Column offset=ColSize::Four size=ColSize::Four>
                      <InputField loading=true />
                    </Column>
                  </Columns>
                </Container>
              </div>
            </Container>
        }
    }
}
