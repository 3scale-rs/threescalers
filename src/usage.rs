use crate::ToParams;

#[derive(Debug, Clone)]
pub struct MetricUsage<'m>(&'m str, &'m str);

impl<'m, M: AsRef<str> + 'm, V: AsRef<str> + 'm> From<&'m (M, V)> for MetricUsage<'m> {
    fn from((m, v): &'m (M, V)) -> Self {
        Self(m.as_ref(), v.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Usage<'m>(Vec<MetricUsage<'m>>);

impl<'m, M: AsRef<str> + 'm, V: AsRef<str> + 'm> From<&'m [(M, V)]> for Usage<'m> {
    fn from(mvs: &'m [(M, V)]) -> Self {
        Self(mvs.iter().map(MetricUsage::from).collect())
    }
}

// Add just enough to be able to access the inner vector for any modifications
impl<'m> Usage<'m> {
    /// Creates a `Usage` using the `From` implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use threescalers::usage::*;
    ///
    /// let mut metrics = Vec::new();
    /// metrics.push(("metric1", "10"));
    /// metrics.push(("metric2", "20"));
    /// let usage = Usage::new(metrics.as_slice());
    /// ```
    pub fn new<M: AsRef<str> + 'm, V: AsRef<str> + 'm>(mv: &'m [(M, V)]) -> Self {
        Self::from(mv)
    }

    /// Consumes the Usage and returns the underlying vector containing metric-value references.
    pub fn into_inner(self) -> Vec<MetricUsage<'m>> {
        self.0
    }

    /// Read access to the underlying vector containing metric-value references.
    pub fn as_vec(&self) -> &Vec<MetricUsage<'m>> {
        self.0.as_ref()
    }

    /// Exclusive access to the underlying vector containing metric-value references.
    pub fn as_mut_vec(&mut self) -> &mut Vec<MetricUsage<'m>> {
        self.0.as_mut()
    }
}

use std::borrow::Cow;

impl<'k, 'v, 'this, E> ToParams<'k, 'v, 'this, E> for Usage<'this>
    where 'this: 'k + 'v,
          E: Extend<(Cow<'k, str>, &'v str)>
{
    fn to_params_with_mangling<F: FnMut(Cow<'k, str>) -> Cow<'k, str>>(&'this self,
                                                                       extendable: &mut E,
                                                                       key_mangling: &mut F) {
        extendable.extend(self.0.iter().map(|mv| {
                                           let m = format!("usage[{}]", mv.0);
                                           (key_mangling(m.into()), mv.1)
                                       }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_params_from_array() {
        let metric1_name = "metric1";
        let metric1_val = "10";
        let metric2_name = "metric2";
        let metric2_val = "20";
        let metrics = [(metric1_name, metric1_val), (metric2_name, metric2_val)];
        let usage = Usage::from(metrics.as_ref());

        let mut result = Vec::new();
        usage.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![("usage[metric1]".into(), metric1_val),
                                                   ("usage[metric2]".into(), metric2_val),];
        assert_eq!(expected, result);
    }

    #[test]
    fn to_params_from_slice() {
        let metric1_name = "metric1";
        let metric1_val = "10";
        let metric2_name = "metric2";
        let metric2_val = "20";
        let mut metrics = Vec::new();
        metrics.push((metric1_name, metric1_val));
        metrics.push((metric2_name, metric2_val));
        let usage = Usage::from(metrics.as_slice());

        let mut result = Vec::new();
        usage.to_params(&mut result);

        let expected: Vec<(Cow<str>, &str)> = vec![("usage[metric1]".into(), metric1_val),
                                                   ("usage[metric2]".into(), metric2_val),];
        assert_eq!(expected, result);
    }
}
