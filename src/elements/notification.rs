use yew::prelude::*;
use yewtil::NeqAssign;

use crate::prelude::*;

pub struct Notification {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    DeleteClicked,
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or(true)]
    pub delete: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub ondelete: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Notification {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Notification { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteClicked => self.props.ondelete.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        let color = self.props.color.is();
        html! {
            <div class=("notification", color, &self.props.class)>
            {
                if self.props.delete {
                    html_nested! { <button class="delete" onclick=self.link.callback(|_| Msg::DeleteClicked)></button> }
                } else {
                    html_nested! {}
                }
            }
            { self.props.children.clone() }
            </div>
        }
    }
}
