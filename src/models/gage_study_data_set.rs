use crate::models::gage_study_data::GageStudyData;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct GageStudyDataSet {
    pub data: Vec<GageStudyData>,
    pub parts: HashSet<String>,
    pub operators: HashSet<u32>,
    pub trials: HashSet<u32>,
    pub study_variation: f64,
    pub tolerance: f64,
    pub mean: f64,
    pub n_part: u32,
    pub n_operator: u32,
    pub n_trial: u32,
    pub n_total: u32,
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
    pub varcomp_part: f64,
    pub varcomp_operator: Option<f64>,
    pub varcomp_total_gagerr: f64,
    pub varcomp_total: f64,
    pub stddev_part: f64,
    pub stddev_operator: Option<f64>,
    pub stddev_total_gagerr: f64,
    pub stddev_total: f64,
    pub p: f64,
}

impl GageStudyDataSet {
    pub fn new() -> Self {
        Self {
            data: Vec::<GageStudyData>::new(),
            parts: HashSet::new(),
            operators: HashSet::new(),
            trials: HashSet::new(),
            study_variation: 5.15,
            tolerance: 0.0,
            mean: 0.0,
            n_part: 0,
            n_operator: 0,
            n_trial: 0,
            n_total: 0,
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
            varcomp_part: 0.0,
            varcomp_operator: None,
            varcomp_total_gagerr: 0.0,
            varcomp_total: 0.0,
            stddev_part: 0.0,
            stddev_operator: None,
            stddev_total_gagerr: 0.0,
            stddev_total: 0.0,
        }
    }

    pub fn new_with_data(data: Vec<GageStudyData>) -> Self {
        let mut gagerr = Self::new();
        gagerr.data = data;
        gagerr
    }

    pub fn calculate_mut(&mut self) -> &mut Self {
        self.mean = self.mean();
        self.parts = self.unique_parts();
        self.n_part = self.parts.len() as u32;
        self.operators = self.unique_operators();
        self.n_operator = self.operators.len() as u32;
        self.trials = self.unique_trials();
        self.n_trial = self.trials.len() as u32;
        self.n_total = self.data.len() as u32;
        self.df_part = self.n_part - 1;
        self.df_operator = self.n_operator - 1;
        self.df_part_operator = self.df_part * self.df_operator;
        self.df_repeatability =
            (self.n_total - 1) - self.df_part - self.df_operator - self.df_part_operator;
        self.df_total = (self.n_part * self.n_operator * self.n_trial) - 1;
        self.ss_part = self.ss_part();
        self.ss_operator = self.ss_operator();
        self.ss_total = self.ss_total();
        self.ss_repeatability = self.ss_repeatability();
        self.ss_part_operator = self.ss_part_operator();
        self.ms_part = self.ms_part();
        self.ms_operator = self.ms_operator();
        self.ms_repeatability = self.ms_repeatability();
        self.ms_part_operator = self.ms_part_operator();
        self.f_part = self.f_part();
        self.f_operator = self.f_operator();
        self.f_part_operator = self.f_part_operator();
        self.p = 0.0;
        self.varcomp_part = self.varcomp_part();
        self.varcomp_operator = self.varcomp_operator();
        self.varcomp_total_gagerr = self.varcomp_total_gagerr();
        self.varcomp_total = self.varcomp_total();
        self.stddev_part = self.varcomp_part.sqrt();
        self.stddev_operator = match self.varcomp_operator {
            Some(vo) => Some(vo.sqrt()),
            None => None,
        };
        self.stddev_total_gagerr = self.varcomp_total_gagerr.sqrt();
        self.stddev_total = self.varcomp_total.sqrt();
        self
    }

    pub fn calculate(mut self) -> Self {
        self.calculate_mut();
        self
    }

    pub fn mean(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.data.len() {
            sum += self.data[i].value;
        }
        sum / self.data.len() as f64
    }

    pub fn unique_parts(&self) -> HashSet<String> {
        let mut parts = HashSet::new();
        for i in self.data.iter() {
            parts.insert(i.part.clone());
        }
        parts
    }

    pub fn unique_operators(&self) -> HashSet<u32> {
        let mut operators = HashSet::new();
        for i in self.data.iter() {
            operators.insert(i.operator);
        }
        operators
    }

    pub fn unique_trials(&self) -> HashSet<u32> {
        let mut trials = HashSet::new();
        for i in self.data.iter() {
            trials.insert(i.trial);
        }
        trials
    }

    pub fn ss_total(&self) -> f64 {
        let mut sum = 0.0;
        for data in self.data.iter() {
            sum += (data.value - self.mean).powi(2);
        }
        sum
    }

    pub fn ss_operator(&self) -> f64 {
        let mut ssdiff = 0.0;

        // Find the operator mean
        let operator_mean = |o: &u32| {
            let mut sum = 0.0;
            let mut count = 0;
            for data in self.data.iter().filter(|d| d.operator == *o) {
                sum += data.value;
                count += 1;
            }
            sum / count as f64
        };

        // Find the sum of square differences between each operator's value and
        // the operator's mean
        for operator in self.operators.iter() {
            ssdiff += (operator_mean(&operator) - self.mean).powi(2);
        }
        ssdiff * (self.n_part * self.n_trial) as f64
    }

    pub fn ss_part(&self) -> f64 {
        let mut ssdiff = 0.0;

        // Find the part mean
        let part_mean = |p: &str| {
            let mut sum = 0.0;
            let mut count: i32 = 0;
            for data in self.data.iter().filter(|d| d.part == p) {
                sum += data.value;
                count += 1;
            }
            sum / count as f64
        };

        // Find the sum of square differences between each part's value and the
        // part's mean
        for part in self.parts.iter() {
            ssdiff += (part_mean(&part) - self.mean).powi(2);
        }
        ssdiff * (self.n_operator * self.n_trial) as f64
    }

    // fn ss_repeatability_interaction(&self) -> f64 {
    //     let trial_mean = |t: &u32| {
    //         let mut sum = 0.0;
    //         let mut count = 0;
    //         for data in self.data.iter().filter(|d| d.trial == *t) {
    //             sum += data.value;
    //             count += 1;
    //         }
    //         sum / count as f64
    //     };

    //     // Find trial means
    //     let mut trial_means = HashMap::new();
    //     for trial in self.trials.iter() {
    //         trial_means.insert(trial, trial_mean(&trial));
    //     }

    //     // Find the sum of square differences between each value within the
    //     // trial and the trial mean
    //     let mut ssdiff = 0.0;
    //     for data in self.data.iter() {
    //         match trial_means.get(&data.trial) {
    //             Some(tm) => ssdiff += (data.value - tm).powi(2),
    //             None => panic!("Trail mean not available"),
    //         }
    //     }
    //     ssdiff
    // }

    fn ss_repeatability(&self) -> f64 {
        if self.n_operator > 1 {
            panic!("Multiple operators not implemented!")
        }
        self.ss_total - self.ss_part - self.ss_operator - self.ss_part_operator
    }

    pub fn ss_part_operator(&self) -> f64 {
        self.ss_total - (self.ss_part + self.ss_operator + self.ss_repeatability)
    }

    // fn df_part(&self) -> u32 {
    //     self.unique_parts().len() as u32 - 1
    // }

    // fn df_operator(&self) -> u32 {
    //     self.unique_operators().len() as u32 - 1
    // }

    // fn df_part_operator(&self) -> u32 {
    //     self.df_part() * self.df_operator()
    // }

    // fn df_repeatability(&self) -> u32 {
    //     let nparts = self.unique_parts().len() as u32;
    //     let noperators = self.unique_operators().len() as u32;
    //     let ntrials = self.unique_trials().len() as u32;
    //     nparts * noperators * (ntrials - 1)
    // }

    // fn df_total(&self) -> u32 {
    //     let nparts = self.unique_parts().len() as u32;
    //     let noperators = self.unique_operators().len() as u32;
    //     let ntrials = self.unique_trials().len() as u32;
    //     (nparts * noperators * ntrials) - 1
    // }

    fn ms_part(&self) -> f64 {
        self.ss_part / self.df_part as f64
    }

    fn ms_operator(&self) -> Option<f64> {
        if self.df_operator == 0 {
            return None;
        }
        Some(self.ss_operator / self.df_operator as f64)
    }

    fn ms_part_operator(&self) -> Option<f64> {
        if self.df_part_operator == 0 {
            return None;
        }
        Some(self.ss_part_operator / self.df_part_operator as f64)
    }

    fn ms_repeatability(&self) -> f64 {
        self.ss_repeatability / self.df_repeatability as f64
    }

    fn f_part(&self) -> f64 {
        self.ms_part / self.ms_repeatability
    }

    fn f_operator(&self) -> Option<f64> {
        match self.ms_operator {
            Some(mso) => Some(mso / self.ms_repeatability),
            None => None,
        }
    }

    fn f_part_operator(&self) -> Option<f64> {
        match self.ms_part_operator {
            Some(mspo) => Some(mspo / self.ms_repeatability),
            _ => None,
        }
    }

    fn varcomp_part(&self) -> f64 {
        return (self.ms_part - self.ms_repeatability) / (self.n_operator * self.n_trial) as f64;
    }

    fn varcomp_operator(&self) -> Option<f64> {
        match self.ms_operator {
            Some(mso) => Some((mso - self.ms_repeatability) / (self.n_part * self.n_trial) as f64),
            None => None,
        }
    }

    fn varcomp_total_gagerr(&self) -> f64 {
        match self.varcomp_operator {
            Some(vo) => self.ms_repeatability + vo,
            None => self.ms_repeatability,
        }
    }

    fn varcomp_total(&self) -> f64 {
        return self.varcomp_total_gagerr + self.varcomp_part;
    }
}
