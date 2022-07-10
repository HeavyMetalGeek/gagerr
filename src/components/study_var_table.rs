use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct StudyVarTableProps {
    pub stddev_part: f64,
    pub stddev_operator: Option<f64>,
    pub stddev_total_gagerr: f64,
    pub stddev_total: f64,
    pub stddev_repeatability: f64,
    pub study_var: f64,
}

impl StudyVarTableProps {
    pub fn new() -> Self {
        Self {
            stddev_part: 0.0,
            stddev_operator: None,
            stddev_total_gagerr: 0.0,
            stddev_total: 0.0,
            stddev_repeatability: 0.0,
            study_var: 0.0,
        }
    }
}

pub struct StudyVarTable {
    pub stddev_part: f64,
    pub stddev_operator: Option<f64>,
    pub stddev_total_gagerr: f64,
    pub stddev_total: f64,
    pub stddev_repeatability: f64,
    pub study_var: f64,
}

impl Component for StudyVarTable {
    type Message = ();
    type Properties = StudyVarTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stddev_part: 0.0,
            stddev_operator: None,
            stddev_total_gagerr: 0.0,
            stddev_total: 0.0,
            stddev_repeatability: 0.0,
            study_var: 0.0,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props().clone();
        self.stddev_part = props.stddev_part;
        self.stddev_operator = props.stddev_operator;
        self.stddev_total_gagerr = props.stddev_total_gagerr;
        self.stddev_total = props.stddev_total;
        self.stddev_repeatability = props.stddev_repeatability;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="data">
                <table class="table">
                    <thead>
                        <tr>
                            <th>{"Source"}</th>
                            <th>{"StdDev (SD)"}</th>
                            <th>{"Study Var ({self.study_var:.4} x SD)"}</th>
                            <th>{"% Study Var"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{"Total Gage R&R"}</td>
                            <td>{format!("{:>-.6}", self.stddev_total_gagerr)}</td>
                            <td>{format!("{:>-.6}", self.stddev_total_gagerr * self.study_var)}</td>
                            <td>{format!("{:>-.2}%", self.stddev_total_gagerr / self.stddev_total * 100.0)}</td>
                        </tr>
                        <tr>
                            <td>{"Repeatability"}</td>
                            <td>{format!("{:>-.6}", self.stddev_repeatability)}</td>
                            <td>{format!("{:>-.6}", self.stddev_repeatability * self.study_var)}</td>
                            <td>{format!("{:>-.2}%", self.stddev_repeatability / self.stddev_total * 100.0)}</td>
                        </tr>
                        <tr>
                            <td>{"Part-To-Part"}</td>
                            <td>{format!("{:>-.6}", self.stddev_part)}</td>
                            <td>{format!("{:>-.6}", self.stddev_part * self.study_var)}</td>
                            <td>{format!("{:>-.2}%", self.stddev_part / self.stddev_total * 100.0)}</td>
                        </tr>
                        <tr>
                            <td>{"Total Variation"}</td>
                            <td>{format!("{:>-.6}", self.stddev_total)}</td>
                            <td>{format!("{:>-.6}", self.stddev_total * self.study_var)}</td>
                            <td>{format!("{:>-.2}%", self.stddev_total / self.stddev_total * 100.0)}</td>
                        </tr>
                    </tbody>
                </table>
                </div>
            </>
        }
    }
}
