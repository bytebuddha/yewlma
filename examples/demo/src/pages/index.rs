use yew::prelude::*;
use yewlma::prelude::*;

pub enum Msg {
    Error,
    Success,
}

pub struct IndexPage(ComponentLink<Self>);

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        IndexPage(link)
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Error => {
                ToastService::default().error("Some Error Message");
            }
            Msg::Success => {
                ToastService::default().success("Some Success Message");
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <Columns>
            <Column size=ColSize::Four offset=ColSize::Eight>
            <Button onsignal=self.0.callback(|_| Msg::Error)>{"Error"}</Button>
            <Button onsignal=self.0.callback(|_| Msg::Success)>{"Success"}</Button>
            </Column>
            </Columns>
        }
    }
}
