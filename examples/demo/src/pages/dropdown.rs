use yew::prelude::*;
use yewlma::prelude::*;

use crate::components::DemoContainer;

pub struct DropDownPage(ComponentLink<Self>);

impl Component for DropDownPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DropDownPage(link)
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
                  <span class="title py-5 px-5">{"Simple"}</span>
                </Columns>
                <Columns>
                  <Column size=ColSize::Half>
                    <DemoContainer>
                      <Buttons>
                          <DropDownMenu items=Self::example_items() />
                      </Buttons>
                    </DemoContainer>
                  </Column>
                </Columns>
              </div>
              <div class="box has-background-color-grey-lighter">
              <Columns>
                <span class="title py-5 px-5">{"Trigger Color"}</span>
              </Columns>
              <Columns>
                <Column size=ColSize::Half>
                  <DemoContainer>
                    <Buttons>
                        <DropDownMenu trigger_color=Color::Primary items=Self::example_items() />
                    </Buttons>
                  </DemoContainer>
                </Column>
              </Columns>
            </div>
            <div class="box has-background-color-grey-lighter">
            <Columns>
              <span class="title py-5 px-5">{"Trigger Outline"}</span>
            </Columns>
            <Columns>
              <Column size=ColSize::Half>
                <DemoContainer>
                  <Buttons>
                      <DropDownMenu trigger_outlined=true
                                    trigger_color=Color::Dark
                                    items=Self::example_items() />
                  </Buttons>
                </DemoContainer>
              </Column>
            </Columns>
          </div>
          <div class="box has-background-color-grey-lighter">
          <Columns>
            <span class="title py-5 px-5">{"Close on Change"}</span>
          </Columns>
          <Columns>
            <Column size=ColSize::Half>
              <DemoContainer>
                <Buttons>
                    <DropDownMenu trigger_outlined=true
                                  trigger_color=Color::Dark
                                  close_on_change=false
                                  items=Self::example_items() />
                </Buttons>
              </DemoContainer>
            </Column>
          </Columns>
        </div>
              </Container>
          }
    }
}

impl DropDownPage {
    fn example_items() -> Vec<DropDownItem> {
        vec![
            DropDownItem::Item {
                text: "Option 1".to_string(),
            },
            DropDownItem::Item {
                text: "Option 2".to_string(),
            },
            DropDownItem::Divider,
            DropDownItem::Item {
                text: "Option 3".to_string(),
            },
            DropDownItem::Item {
                text: "Option 4".to_string(),
            },
        ]
    }
}
