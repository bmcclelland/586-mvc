use yew::prelude::*;
use crate::model::*;
use crate::msg::*;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model::new(properties, link)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        update(self, msg)
    }
}
