use crate::components::anova_table::AnovaTable;
use crate::components::list_item::ListItem;
use crate::components::study_var_table::StudyVarTable;
use crate::components::varcomp_table::VarCompTable;
use crate::models::gage_data::GageData;
use crate::models::gage_study_data::GageStudyData;
use crate::models::gage_study_data_set::GageStudyDataSet;
use csv::Reader;
use gloo_file::{callbacks::FileReader, File};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    File(Option<File>),
    Loaded(String, String),
    ClearFile,
}

#[derive(Properties, PartialEq)]
pub struct ListTableProps {
    pub items: Vec<GageData>,
}

pub struct ListTable {
    pub items: Vec<GageData>,
    pub raw_data: String,
    pub readers: HashMap<String, FileReader>,
    pub anova_data: GageStudyDataSet,
}

impl Component for ListTable {
    type Message = Msg;
    type Properties = ListTableProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            raw_data: String::new(),
            readers: HashMap::new(),
            anova_data: GageStudyDataSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(fname, data) => {
                self.items = Vec::new();
                self.raw_data = data;
                log::info!("File loaded...");
                self.readers.remove(&fname);
                let mut rdr = Reader::from_reader(self.raw_data.as_bytes());
                let iter = rdr.deserialize();
                for result in iter {
                    match result {
                        Ok(r) => self.items.push(r),
                        Err(e) => log::info!("{e}"),
                    }
                }
                log::info!("Items loaded: {}", self.items.len());
                self.anova_data = self.process_gage_study();
                true
            }
            Msg::File(f) => {
                let link = ctx.link().clone();
                if let Some(c) = f {
                    let fname = c.name();
                    let task = gloo_file::callbacks::read_as_text(&c, move |res| {
                        link.send_message(Msg::Loaded(fname, res.unwrap_or_else(|e| e.to_string())))
                    });
                    self.readers.insert(c.name(), task);
                }
                true
            }
            Msg::ClearFile => {
                self.raw_data = String::new();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dummy = ctx.link().callback(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            match input {
                Some(i) => {
                    let file = i.files().unwrap().get(0).map(File::from);
                    Msg::File(file)
                }
                _ => Msg::ClearFile,
            }
        });
        html! {
            <>
            <div class="container">
                <input type="file" id="csvinput" name="csvinput" accept=".csv" onchange={dummy}/>
            </div>
            <div class="data">
                <table class="list-table">
                    <tr>
                        <th>{"Part"}</th>
                        <th>{"Operator"}</th>
                        <th>{"Trial"}</th>
                        <th>{"Value"}</th>
                    </tr>
                    { for self.items
                        .iter()
                        .map(|col| html_nested!{
                            <ListItem message={col.clone()} />
                        })
                    }
                </table>
            </div>
            <AnovaTable
                df_part={self.anova_data.df_part}
                df_operator={self.anova_data.df_operator}
                df_part_operator={self.anova_data.df_part_operator}
                df_repeatability={self.anova_data.df_repeatability}
                df_total={self.anova_data.df_total}
                ss_part={self.anova_data.ss_part}
                ss_operator={self.anova_data.ss_operator}
                ss_part_operator={self.anova_data.ss_part_operator}
                ss_repeatability={self.anova_data.ss_repeatability}
                ss_total={self.anova_data.ss_total}
                ms_part={self.anova_data.ms_part}
                ms_operator={self.anova_data.ms_operator}
                ms_repeatability={self.anova_data.ms_repeatability}
                f_part={self.anova_data.f_part}
                f_operator={self.anova_data.f_operator}
                f_part_operator={self.anova_data.f_part_operator}
                p={self.anova_data.p}
            />
            <VarCompTable
                varcomp_part={self.anova_data.varcomp_part}
                varcomp_operator={self.anova_data.varcomp_operator}
                varcomp_repeatability={self.anova_data.ms_repeatability}
                varcomp_total_gagerr={self.anova_data.varcomp_total_gagerr}
                varcomp_total={self.anova_data.varcomp_total}
            />
            <StudyVarTable
                stddev_part={self.anova_data.varcomp_part.sqrt()}
                stddev_operator={
                    match self.anova_data.varcomp_operator {
                        Some(v) => Some(v.sqrt()),
                        None => None
                    }
                }
                stddev_repeatability={self.anova_data.ms_repeatability.sqrt()}
                stddev_total_gagerr={self.anova_data.ms_repeatability.sqrt() + self.anova_data.varcomp_operator.unwrap_or(0.0).sqrt()}
                stddev_total={self.anova_data.varcomp_total.sqrt()}
                study_var={self.anova_data.study_variation}
            />
            </>
        }
    }
}

impl ListTable {
    pub fn process_gage_study(&self) -> GageStudyDataSet {
        let data = self
            .items
            .iter()
            .map(|i| GageStudyData::new(&i.part, i.operator, i.trial, i.value))
            .collect();
        GageStudyDataSet::new_with_data(data).calculate()
    }
}
