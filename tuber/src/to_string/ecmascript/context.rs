use super::function;
use crate::context::Context;
use crate::func::Func;
use regex::Regex;

pub fn to_string(context: &Context) -> String {
    let mut vec = context
        .iter()
        .map(|(_, func)| feature(func))
        .collect::<Vec<(Feature, &Func)>>();

    vec.sort_by(|l, r| {
        let l = &l.0;
        let r = &r.0;
        r.short
            .cmp(&l.short)
            .then(l.name.to_lowercase().cmp(&r.name.to_lowercase()))
            .then(r.name.cmp(l.name))
            .then(l.index.cmp(&r.index))
    });

    vec.into_iter()
        .map(|(_, func)| function::to_string(func))
        .collect::<Vec<_>>()
        .join("\n")
}

struct Feature<'a> {
    short: bool,
    name: &'a str,
    index: Option<usize>,
}

fn feature(func: &Func) -> (Feature<'_>, &Func) {
    let pattern = Regex::new(r"\A(.*?)(\d*)\z").unwrap();

    let name = func.name();
    let (name, index) = match pattern.captures(name).map(|c| c.extract()) {
        Some((_, [s, n])) => (s, n.parse::<usize>().ok()),
        None => (name, None),
    };

    let short = index.is_none() && name.len() == 1;

    (Feature { short, name, index }, func)
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func;
    use rand::seq::SliceRandom;

    #[test]
    fn test_to_string() {
        let mut functions = [
            func::new("i", vec!["x"], "x"),
            func::new("k", vec!["x", "y"], "x"),
            func::new("K", vec!["x", "y"], "x"),
            func::new("l", vec!["x", "y"], "x"),
        ];

        // functions を事前にシャッフルしてから Context を作る
        // これによって Context の印字が Func の順序依存でないことを確かめる

        let mut rng = rand::thread_rng();
        functions.shuffle(&mut rng);

        let context = Context::from(functions.to_vec());

        assert_eq!(
            to_string(&context),
            "
                i(x) = x\n\
                k(x, y) = x\n\
                K(x, y) = x\n\
                l(x, y) = x
            "
            .trim()
        );
    }
}
