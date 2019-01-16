use crate::ToParams;

#[derive(Debug)]
pub struct Usage {
    metrics: Vec<(String, String)>,
    usage_params: Vec<(String, String)>,
}

// TODO fix this to stop the wild cloning of strings
// IMO we should treat Usage as a container itself and share it in multiple calls
// ie.
// let u = Usage::new();
// u.push(("metric1", 10));
// u.push(("metric2", 20));
// We should also be able to cache the generated parameters, and turn into the inner container,
// clone, iterate, etc.
// An internal Vec should do, as we are unlikely to need direct access to specific entries once we
// have a usage (we can do it anyway, but searching sequentially). If we needed something more
// powerful we can always write conversions from Vec/HashMap to Usage.
impl Usage {
    /// Creates a `Usage`.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::usage::*;
    ///
    /// let mut metrics = Vec::new();
    /// metrics.push(("metric1", 10));
    /// metrics.push(("metric2", 20));
    /// let usage = Usage::new(&metrics);
    /// ```
    pub fn new<T1: ToString, T2: ToString>(metrics: &[(T1, T2)]) -> Self {
        let string_metrics = metrics
            .iter()
            .map(|&(ref metric, ref value)| (metric.to_string(), value.to_string()))
            .collect();

        Self {
            metrics: string_metrics,
            usage_params: Usage::usage_params_from(metrics),
        }
    }

    fn usage_params_from<T1: ToString, T2: ToString>(metrics: &[(T1, T2)])
                                                     -> Vec<(String, String)> {
        metrics
            .iter()
            .map(|&(ref metric, ref value)| {
                     (Usage::param_name(&metric.to_string()), value.to_string())
                 })
            .collect()
    }

    fn param_name(param: &str) -> String {
        "usage[".to_owned() + param + "]"
    }
}

impl<'k, 'v, E> ToParams<'k, 'v, E> for Usage where E: Extend<(&'k str, &'v str)> {
    fn to_params<'s: 'k + 'v>(&'s self, extendable: &mut E) {
        extendable.extend(
            self.usage_params
                .iter()
                .map(|&(ref metric, ref value)|
                    (metric.as_str(), value.as_str())
                )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_transformed_into_params() {
        let metric1_name = "metric1";
        let metric1_val = "10";
        let metric2_name = "metric2";
        let metric2_val = "20";
        let mut metrics = Vec::new();
        metrics.push((metric1_name, metric1_val));
        metrics.push((metric2_name, metric2_val));
        let usage = Usage::new(&metrics);

        let mut result = Vec::new();
        usage.to_params(&mut result);

        let expected = vec![("usage[metric1]", metric1_val),
                            ("usage[metric2]", metric2_val)];
        assert_eq!(expected, result);
    }
}
