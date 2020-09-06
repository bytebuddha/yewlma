use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type Buttons = Pure<PureButtons>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureButtons {
    #[prop_or_default]
    pub addons: bool,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub right: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for PureButtons {
    fn render(&self) -> Html {
        let addons = self.addons.then_some("has-addons");
        let right = self.right.then_some("is-right");
        let center = self.center.then_some("is-centered");
        html! {
            <div class=("buttons", addons, right, center, &self.class)>
              { self.children.clone() }
            </div>
        }
    }
}
