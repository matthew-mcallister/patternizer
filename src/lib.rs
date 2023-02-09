use cpython::{exc, py_fn, py_module_initializer, PyErr, PyResult, Python};
use regex::Regex;

py_module_initializer!(patternizer, |py, m| {
    m.add(py, "__doc__", "Implements performant regex evaluation.")?;
    m.add(
        py,
        "eval_regex",
        py_fn!(py, eval_regex_py(regex: String, samples: Vec<String>)),
    )?;
    Ok(())
});

fn eval_single_regex(regex: &Regex, sample: &str) -> f32 {
    if let Some(captures) = regex.captures(sample) {
        let size: usize = captures
            .iter()
            .map(|x| x.map_or(0, |cap| cap.end() - cap.start()))
            .sum();
        size as f32 / sample.len() as f32
    } else {
        0f32
    }
}

fn eval_regex<'a>(regex: &Regex, samples: impl IntoIterator<Item = &'a str>) -> Vec<f32> {
    samples
        .into_iter()
        .map(|s| eval_single_regex(regex, s))
        .collect()
}

fn eval_regex_py(py: Python<'_>, regex: String, samples: Vec<String>) -> PyResult<Vec<f32>> {
    let regex =
        Regex::new(&regex).map_err(|e| PyErr::new::<exc::ValueError, _>(py, e.to_string()))?;
    Ok(eval_regex(&regex, samples.iter().map(|s| &s[..])))
}
