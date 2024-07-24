extern crate core;

use core::models::monte_carlo::MonteCarloModel;
use core::models::OptionPricingModel;

   #[test]
    fn test_call_price() {
        let model = MonteCarloModel { simulations: 100000 };
        let call_price = model.call_price(100.0, 100.0, 0.05, 0.2, 1.0);
        assert!((call_price - 10.45).abs() < 1.0);
    }

    #[test]
    fn test_put_price() {
        let model = MonteCarloModel { simulations: 100000 };
        let put_price = model.put_price(100.0, 100.0, 0.05, 0.2, 1.0);
        assert!((put_price - 5.57).abs() < 1.0);
    }

