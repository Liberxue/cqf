use crate::models::{OptionParameters, OptionPricingModel};

pub struct BinomialTreeModel;

// https://www.kent.ac.uk/learning/documents/slas-documents/Binomial_models.pdf
// https://www.le.ac.uk/users/dsgp1/COURSES/DERIVATE/BINOPTION.PDF
impl OptionPricingModel for BinomialTreeModel {
    fn call_price(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }
    fn put_price(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }

    fn delta(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }

    fn gamma(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }

    fn theta(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }

    fn vega(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }

    fn rho(&self, _params: &OptionParameters) -> f64 {
        unimplemented!()
    }
}
