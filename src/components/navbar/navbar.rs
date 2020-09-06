use crate::prelude::*;
use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type NavBar = Pure<PureNavBar>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureNavBar {
    #[prop_or("nav")]
    pub tag: &'static str,
    #[prop_or_default]
    pub shadow: bool,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for PureNavBar {
    fn render(&self) -> Html {
        let shadow = self.shadow.then_some("is-shadowed");
        let color = self.color.is();
        html! {
            <@{self.tag} class=("navbar", color, shadow)>
              <Container>
                { self.children.clone() }
              </Container>
            </@>
        }
    }
}
