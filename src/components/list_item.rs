use crate::models::gage_data::GageData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub message: GageData,
}

pub struct ListItem {
    pub part: String,
    pub operator: u32,
    pub trial: u32,
    pub value: f64,
}

impl Component for ListItem {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            part: "".to_owned(),
            operator: 0,
            trial: 0,
            value: 0.0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <tr>
                <td class="list-item-part">{&ctx.props().message.part}</td>
                <td class="list-item-operator">{&ctx.props().message.operator}</td>
                <td class="list-item-trial">{&ctx.props().message.trial}</td>
                <td class="list-item-value">{format!("{:>-.6}",&ctx.props().message.value)}</td>
            </tr>
        }
    }
}
