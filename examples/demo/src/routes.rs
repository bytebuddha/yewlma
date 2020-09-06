use yew_router::prelude::*;

#[derive(Debug, PartialEq, Clone, Switch)]
pub enum AppRoutes {
    #[to = "/toaster"]
    Toaster,
    #[to = "/notifications"]
    Notifications,
    #[to = "/input"]
    Input,
    #[to = "/layout"]
    Layout,
    #[to = "/dropdown"]
    DropDown,
    #[to = "/buttons"]
    Buttons,
    #[to = "/"]
    Index,
}
