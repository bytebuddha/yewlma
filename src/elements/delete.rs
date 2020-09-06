use crate::prelude::*;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Delete(Props, ComponentLink<Self>);

pub enum Msg {
    Clicked,
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub onsignal: Callback<()>,
}

impl Component for Delete {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Delete(props, link)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => self.0.onsignal.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        let size = self.0.size.is();
        let color = self.0.color.has_bg();
        html! {
            <a onclick=self.1.callback(|_| Msg::Clicked)
               class=("delete", size, color, &self.0.class)></a>
        }
    }
}
