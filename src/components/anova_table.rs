use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct AnovaTableProps {
    pub df_part: u32,
    pub df_operator: u32,
    pub df_part_operator: u32,
    pub df_repeatability: u32,
    pub df_total: u32,
    pub ss_part: f64,
    pub ss_operator: f64,
    pub ss_part_operator: f64,
    pub ss_repeatability: f64,
    pub ss_total: f64,
    pub ms_part: f64,
    pub ms_operator: Option<f64>,
    pub ms_part_operator: Option<f64>,
    pub ms_repeatability: f64,
    pub f_part: f64,
    pub f_operator: Option<f64>,
    pub f_part_operator: Option<f64>,
    pub p: f64,
}

impl AnovaTableProps {
    pub fn new() -> Self {
        Self {
            df_part: 0,
            df_operator: 0,
            df_part_operator: 0,
            df_repeatability: 0,
            df_total: 0,
            ss_part: 0.0,
            ss_operator: 0.0,
            ss_part_operator: 0.0,
            ss_repeatability: 0.0,
            ss_total: 0.0,
            ms_part: 0.0,
            ms_operator: None,
            ms_part_operator: None,
            ms_repeatability: 0.0,
            f_part: 0.0,
            f_operator: None,
            f_part_operator: None,
            p: 0.0,
        }
    }
}

pub struct AnovaTable {
    pub df_part: u32,
    pub df_operator: u32,
    pub df_part_operator: u32,
    pub df_repeatability: u32,
    pub df_total: u32,
    pub ss_part: f64,
    pub ss_operator: f64,
    pub ss_part_operator: f64,
    pub ss_repeatability: f64,
    pub ss_total: f64,
    pub ms_part: f64,
    pub ms_operator: Option<f64>,
    pub ms_part_operator: Option<f64>,
    pub ms_repeatability: f64,
    pub f_part: f64,
    pub f_operator: Option<f64>,
    pub f_part_operator: Option<f64>,
    pub p: f64,
}

impl Component for AnovaTable {
    type Message = ();
    type Properties = AnovaTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            df_part: 0,
            df_operator: 0,
            df_part_operator: 0,
            df_repeatability: 0,
            df_total: 0,
            ss_part: 0.0,
            ss_operator: 0.0,
            ss_part_operator: 0.0,
            ss_repeatability: 0.0,
            ss_total: 0.0,
            ms_part: 0.0,
            ms_operator: None,
            ms_part_operator: None,
            ms_repeatability: 0.0,
            f_part: 0.0,
            f_operator: None,
            f_part_operator: None,
            p: 0.0,
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props().clone();
        self.df_part = props.df_part;
        self.df_part_operator = props.df_part_operator;
        self.df_repeatability = props.df_repeatability;
        self.df_total = props.df_total;
        self.ss_part = props.ss_part;
        self.ss_operator = props.ss_operator;
        self.ss_part_operator = props.ss_part_operator;
        self.ss_repeatability = props.ss_repeatability;
        self.ms_part = props.ms_part;
        self.ms_operator = props.ms_operator;
        self.ms_part_operator = props.ms_part_operator;
        self.ms_repeatability = props.ms_repeatability;
        self.f_part = props.f_part;
        self.f_operator = props.f_operator;
        self.f_part_operator = props.f_part_operator;
        self.p = props.p;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="data">
                    <table class="anova-table">
                        <tr>
                            <th>{"Source"}</th>
                            <th>{"DF"}</th>
                            <th>{"SS"}</th>
                            <th>{"MS"}</th>
                            <th>{"F"}</th>
                            <th>{"P"}</th>
                        </tr>
                        <tr>
                            <td>{"Parts"}</td>
                            <td>{format!("{:>-.6}", self.df_part)}</td>
                            <td>{format!("{:>-.6}", self.ss_part)}</td>
                            <td>{format!("{:>-.6}", self.ms_part)}</td>
                            <td>{format!("{:>-.6}", self.f_part)}</td>
                            <td>{format!("{:>-.6}", self.p)}</td>
                        </tr>
                        if self.df_operator > 2 {
                            <tr>
                                <td>{"Operator"}</td>
                                <td>{format!("{:>-.6}", self.df_operator)}</td>
                                <td>{format!("{:>-.6}", self.ss_operator)}</td>
                                <td>{format!("{:>-.6}", self.ms_operator.unwrap_or(0.0))}</td>
                                <td>{format!("{:>-.6}", self.f_operator.unwrap_or(0.0))}</td>
                                <td>{""}</td>
                            </tr>
                            <tr>
                                <td>{"Parts * Operator"}</td>
                                <td>{format!("{:>-.6}", self.df_part_operator)}</td>
                                <td>{format!("{:>-.6}", self.ss_part_operator)}</td>
                                <td>{format!("{:>-.6}", self.ms_part_operator.unwrap_or(0.0))}</td>
                                <td>{format!("{:>-.6}", self.f_part_operator.unwrap_or(0.0))}</td>
                                <td>{""}</td>
                            </tr>
                        }
                        <tr>
                            <td>{"Repeatability"}</td>
                            <td>{format!("{:>-.6}", self.df_repeatability)}</td>
                            <td>{format!("{:>-.6}", self.ss_repeatability)}</td>
                            <td>{format!("{:>-.6}", self.ms_repeatability)}</td>
                            <td>{""}</td>
                            <td>{""}</td>
                        </tr>
                        <tr>
                            <td>{"Total"}</td>
                            <td>{format!("{:>-.6}", self.df_total)}</td>
                            <td>{format!("{:>-.6}", self.ss_total)}</td>
                            <td>{""}</td>
                            <td>{""}</td>
                            <td>{""}</td>
                        </tr>
                    </table>
                </div>
            </>
        }
    }
}
