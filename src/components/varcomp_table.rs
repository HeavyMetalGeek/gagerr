use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct VarCompTableProps {
    pub varcomp_part: f64,
    pub varcomp_operator: Option<f64>,
    pub varcomp_total_gagerr: f64,
    pub varcomp_total: f64,
    pub varcomp_repeatability: f64,
}

impl VarCompTableProps {
    pub fn new() -> Self {
        Self {
            varcomp_part: 0.0,
            varcomp_operator: None,
            varcomp_total_gagerr: 0.0,
            varcomp_total: 0.0,
            varcomp_repeatability: 0.0,
        }
    }
}

pub struct VarCompTable {
    pub varcomp_part: f64,
    pub varcomp_operator: Option<f64>,
    pub varcomp_total_gagerr: f64,
    pub varcomp_total: f64,
    pub varcomp_repeatability: f64,
}

impl Component for VarCompTable {
    type Message = ();
    type Properties = VarCompTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            varcomp_part: 0.0,
            varcomp_operator: None,
            varcomp_total_gagerr: 0.0,
            varcomp_total: 0.0,
            varcomp_repeatability: 0.0,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props().clone();
        self.varcomp_part = props.varcomp_part;
        self.varcomp_operator = props.varcomp_operator;
        self.varcomp_total_gagerr = props.varcomp_total_gagerr;
        self.varcomp_total = props.varcomp_total;
        self.varcomp_repeatability = props.varcomp_repeatability;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="data">
                <div class="table-header"><h2>{"Variance Components"}</h2></div>
                <table class="varcomp-table">
                    <thead>
                        <tr>
                            <th>{"Source"}</th>
                            <th>{"VarComp"}</th>
                            <th>{"% Contribution"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{"Total Gage R&R"}</td>
                            <td>{format!("{:>-.6}", self.varcomp_total_gagerr)}</td>
                            <td>{format!("{:>-.2}%", self.varcomp_total_gagerr / self.varcomp_total * 100.0)}</td>
                        </tr>
                        <tr>
                            <td>{"Repeatability"}</td>
                            <td>{format!("{:>-.6}", self.varcomp_repeatability)}</td>
                            <td>{format!("{:>-.2}%", self.varcomp_repeatability / self.varcomp_total * 100.0)}</td>
                        </tr>
                        <tr>
                            <td>{"Part-To-Part"}</td>
                            <td>{format!("{:>-.6}", self.varcomp_part)}</td>
                            <td>{format!("{:>-.2}%", self.varcomp_part / self.varcomp_total * 100.0)}</td>
                        </tr>
                        <tr>
                            <td>{"Total Variation"}</td>
                            <td>{format!("{:>-.6}", self.varcomp_total)}</td>
                            <td>{format!("{:>-.2}%", self.varcomp_total / self.varcomp_total * 100.0)}</td>
                        </tr>
                    </tbody>
                </table>
                </div>
            </>
        }
    }
}
