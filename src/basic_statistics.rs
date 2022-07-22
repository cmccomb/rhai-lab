use rhai::plugin::*;
use rhai::EvalAltResult;

#[export_module]
pub mod stats {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

    /// Return the highest value from a pair of numbers.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let the_higher_number = max(2, 3);
    /// assert_eq(the_higher_number, 3);
    /// ```
    /// ```rhai
    /// let the_higher_number = max(2.0, 3.0);
    /// assert_eq(the_higher_number, 3.0);
    /// ```
    #[rhai_fn(name = "max")]
    pub fn gen_max(a: Dynamic, b: Dynamic) -> Dynamic {
        array_max(vec![a, b]).unwrap()
    }

    /// Return the highest value from an array.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let the_highest_number = max([2, 3, 4, 5]);
    /// assert_eq(the_highest_number, 5);
    /// ```
    #[rhai_fn(name = "max", return_raw)]
    pub fn array_max(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Ok(Dynamic::from(y[y.len() - 1]))
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            Ok(Dynamic::from(y[y.len() - 1]))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Return the lowest value from a pair of numbers.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let the_higher_number = max(2, 3);
    /// assert_eq(the_higher_number, 2);
    /// ```
    /// ```rhai
    /// let the_higher_number = max(2.0, 3.0);
    /// assert_eq(the_higher_number, 2.0);
    /// ```
    #[rhai_fn(name = "min")]
    pub fn gen_min(a: Dynamic, b: Dynamic) -> Dynamic {
        array_min(vec![a, b]).unwrap()
    }

    /// Return the lowest value from an array.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let the_lowest_number = max([2, 3, 4, 5]);
    /// assert_eq(the_lowest_number, 2);
    /// ```
    #[rhai_fn(name = "min", return_raw)]
    pub fn array_min(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Ok(Dynamic::from(y[0]))
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            Ok(Dynamic::from(y[0]))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Return the highest value from an array.
    ///
    /// # Example
    ///
    /// ```rhai
    /// let high_and_low = bounds([2, 3, 4, 5]);
    /// assert_eq(high_and_low, [2, 5]);
    /// ```
    #[rhai_fn(name = "bounds")]
    pub fn bounds(arr: Array) -> Array {
        vec![
            Dynamic::from(array_min(arr.clone()).unwrap()),
            Dynamic::from(array_max(arr.clone()).unwrap()),
        ]
    }

    /// Returns the `k` highest values from an array.
    /// ```javascript
    /// let data = [32, 15, -7, 10, 1000, 41, 42];
    /// let mk = maxk(data, 3);
    /// assert_eq(mk, [41, 42, 1000]);
    /// ```
    #[rhai_fn(name = "maxk", return_raw)]
    pub fn maxk(arr: Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let r = (y.len() - (k as usize))..(y.len());
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            let r = (y.len() - (k as usize))..(y.len());
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    ///
    /// ```rhai
    /// let data = [32, 15, -7, 10, 1000, 41, 42];
    /// let mk = mink(data, 3);
    /// assert_eq(mk, [-7, 10, 15]);
    /// ```
    #[rhai_fn(name = "mink", return_raw)]
    pub fn mink(arr: Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let r = (0 as usize)..(k as usize);
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            let r = (0 as usize)..(k as usize);
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }
}