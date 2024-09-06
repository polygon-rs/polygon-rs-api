use crate::{ErrorCode, Parameter, ParameterRequirment, Parameters, Request, Sort, Timespan};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Grouped {
    grouped_parameters: Option<Parameters>,
    grouped_url: Option<String>,
    pub adjusted: bool,
    pub results: Vec<Bar>,
    pub status: String,
    pub resultsCount: i32,
    pub query_count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bar {
    pub T: String,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub n: i32,
    pub o: f64,
    pub t: i64,
    pub v: f64,
    pub vw: f64,
}

impl Grouped {
    pub fn set_parameters(
        &mut self,
        api_key: String,
        date: String,
        include_otc: Option<bool>,
        adjusted: Option<bool>,
    ) {
        self.grouped_parameters = Some(Parameters {
            api_key: api_key,
            date: Some(date),
            adjusted: adjusted,
            include_otc: include_otc,
            ..Parameters::default()
        })
    }
}

impl Request for Grouped {
    const VERSION: &'static str = "v2";
    const CALL: &'static str = "aggs/grouped";
    const PARAMETERS: &'static [&'static ParameterRequirment] = &[
        &ParameterRequirment {
            required: true,
            parameter: Parameter::Date,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::Adjusted,
        },
        &ParameterRequirment {
            required: false,
            parameter: Parameter::IncludeOTC,
        },
    ];

    fn parameters(&self) -> &Parameters {
        match &self.grouped_parameters {
            Some(p) => p,
            None => panic!("There is no parameters set"),
        }
    }

    fn url(&mut self) -> String {
        self.set_url();
        match &self.grouped_url {
            Some(u) => u.to_string(),
            None => panic!("There is no url set"),
        }
    }

    fn set_url(&mut self) {
        self.check_parameters();
        self.grouped_url = Some(String::from(format!(
            "{}/{}/{}/locale/us/market/stocks/{}?{}{}apiKey={}",
            Self::BASE_URL,
            Self::VERSION,
            Self::CALL,
            self.parameters().clone().date.unwrap(),
            if let Some(adj) = self.parameters().clone().adjusted {
                format!("adjusted={}&", adj)
            } else {
                "".to_string()
            },
            if let Some(s) = self.parameters().clone().include_otc {
                format!("include_otc={}&", s)
            } else {
                "".to_string()
            },
            self.parameters().clone().api_key,
        )));
    }

    fn request(&mut self) -> Result<(), ErrorCode> {
        let r = match self.get_raw_data() {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let a: Grouped = match serde_json::from_str(r.as_str()) {
            Ok(it) => it,
            Err(err) => return Err(ErrorCode::FormatError),
        };
        *self = a;

        Ok(())
    }
}
