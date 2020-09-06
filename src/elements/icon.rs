use yew::prelude::*;
use yewtil::{Pure, PureComponent};

use crate::prelude::*;

pub type Icon = Pure<PureIcon>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureIcon {
    pub name: String,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub class: Option<String>,
}

impl PureComponent for PureIcon {
    fn render(&self) -> Html {
        let size = self.size.is();
        let color = self.color.has_text();
        html! {
            <a class=("icon", size, color, &self.class)>
              <i class=&self.name></i>
            </a>
        }
    }
}
