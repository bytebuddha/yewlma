use yew::prelude::*;
use yewlma::prelude::*;

pub struct LayoutPage(ComponentLink<Self>);

impl Component for LayoutPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        LayoutPage(link)
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
                  <span class="title py-5 px-5">{"Width"}</span>
                </Columns>
                <Container>
                  <Columns>
                    <Column size=ColSize::One>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"One"}</div>
                      </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Two>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Two"}</div>
                      </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Three>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Three"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Four>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Four"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Five>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"One"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Six>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Six"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Seven>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Seven"}</div>
                      </Card>
                    </Column>
                    <Column>
                        <Card bg=Color::Primary>
                            <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                        </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Eight>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Eight"}</div>
                      </Card>
                    </Column>
                    <Column>
                        <Card bg=Color::Primary>
                            <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                        </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Nine>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Nine"}</div>
                     </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Ten>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Ten"}</div>
                     </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::Eleven>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Eleven"}</div>
                      </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                  <Column size=ColSize::Twelve>
                    <Card bg=Color::Info fg=Color::White>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Twelve"}</div>
                    </Card>
                  </Column>
                  </Columns>
                </Container>
              </div>
              <div class="box has-background-color-grey-lighter">
                <Columns>
                  <span class="title py-5 px-5">{"Offset"}</span>
                </Columns>
                <Container>
                  <Columns>
                    <Column size=ColSize::One>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Zero"}</div>
                      </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::One>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"One"}</div>
                      </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Two>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Two"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Three>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Three"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Four>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Four"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Five>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Five"}</div>
                      </Card>
                    </Column>
                    <Column>
                    <Card bg=Color::Primary>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                    </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Six>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Six"}</div>
                      </Card>
                    </Column>
                    <Column>
                        <Card bg=Color::Primary>
                            <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                        </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Seven>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Seven"}</div>
                      </Card>
                    </Column>
                    <Column>
                        <Card bg=Color::Primary>
                            <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                        </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Eight>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Eight"}</div>
                     </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Nine>
                      <Card fg=Color::White bg=Color::Info>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Nine"}</div>
                     </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                    <Column size=ColSize::One offset=ColSize::Ten>
                      <Card bg=Color::Info fg=Color::White>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Ten"}</div>
                      </Card>
                    </Column>
                    <Column>
                      <Card bg=Color::Primary>
                        <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Auto"}</div>
                      </Card>
                    </Column>
                  </Columns>
                  <Columns>
                  <Column size=ColSize::One offset=ColSize::Eleven>
                    <Card bg=Color::Info fg=Color::White>
                      <div class="card-content is-size-7 has-text-centered" style="padding:0.5em">{"Eleven"}</div>
                    </Card>
                  </Column>
                  </Columns>
                </Container>
              </div>
            </Container>
        }
    }
}
