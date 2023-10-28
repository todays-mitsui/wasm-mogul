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
    fn test_to_string_1() {
        let mut functions = [
            func::new("i", vec!["x", "y"], "x"),
            func::new("j", vec!["x", "y"], "x"),
            func::new("k", vec!["x", "y"], "x"),
            func::new("I", vec!["x", "y"], "x"),
            func::new("J", vec!["x", "y"], "x"),
            func::new("K", vec!["x", "y"], "x"),
        ];

        // functions を事前にシャッフルしてから Context を作る
        // これによって Context の印字が Func の順序依存でないことを確かめる
        let mut rng = rand::thread_rng();
        functions.shuffle(&mut rng);

        let context = Context::from(functions.to_vec());

        assert_eq!(
            to_string(&context),
            "
                ``ixy = x\n\
                ``Ixy = x\n\
                ``jxy = x\n\
                ``Jxy = x\n\
                ``kxy = x\n\
                ``Kxy = x
            "
            .trim()
        );
    }

    #[test]
    fn test_to_string_2() {
        let mut functions = [
            func::new("i", vec!["x", "y"], "x"),
            func::new("j", vec!["x", "y"], "x"),
            func::new("I", vec!["x", "y"], "x"),
            func::new("J", vec!["x", "y"], "x"),
            func::new("IX", vec!["x", "y"], "x"),
            func::new("JX", vec!["x", "y"], "x"),
        ];

        // functions を事前にシャッフルしてから Context を作る
        // これによって Context の印字が Func の順序依存でないことを確かめる
        let mut rng = rand::thread_rng();
        functions.shuffle(&mut rng);

        let context = Context::from(functions.to_vec());

        assert_eq!(
            to_string(&context),
            "
                ``ixy = x\n\
                ``Ixy = x\n\
                ``jxy = x\n\
                ``Jxy = x\n\
                ``IXxy = x\n\
                ``JXxy = x
            "
            .trim()
        );
    }

    #[test]
    fn test_to_string_3() {
        let mut functions = [
            func::new("a", vec!["x", "y"], "x"),
            func::new("A", vec!["x", "y"], "x"),
            func::new("A0", vec!["x", "y"], "x"),
            func::new("A1", vec!["x", "y"], "x"),
        ];

        // functions を事前にシャッフルしてから Context を作る
        // これによって Context の印字が Func の順序依存でないことを確かめる
        let mut rng = rand::thread_rng();
        functions.shuffle(&mut rng);

        let context = Context::from(functions.to_vec());

        assert_eq!(
            to_string(&context),
            "
                ``axy = x\n\
                ``Axy = x\n\
                ``A0xy = x\n\
                ``A1xy = x
            "
            .trim()
        );
    }

    #[test]
    fn test_to_string_4() {
        let mut functions = [
            func::new("1", vec!["x", "y"], "x"),
            func::new("2", vec!["x", "y"], "x"),
            func::new("10", vec!["x", "y"], "x"),
            func::new("20", vec!["x", "y"], "x"),
            func::new("X1", vec!["x", "y"], "x"),
            func::new("X2", vec!["x", "y"], "x"),
            func::new("X10", vec!["x", "y"], "x"),
            func::new("X20", vec!["x", "y"], "x"),
        ];

        // functions を事前にシャッフルしてから Context を作る
        // これによって Context の印字が Func の順序依存でないことを確かめる
        let mut rng = rand::thread_rng();
        functions.shuffle(&mut rng);

        let context = Context::from(functions.to_vec());

        assert_eq!(
            to_string(&context),
            "
                ``1xy = x\n\
                ``2xy = x\n\
                ``10xy = x\n\
                ``20xy = x\n\
                ``X1xy = x\n\
                ``X2xy = x\n\
                ``X10xy = x\n\
                ``X20xy = x
            "
            .trim()
        );
    }
}
