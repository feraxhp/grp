use color_print::cformat;

/// vec: [>=1]
/// - 0 : org
/// - 1+: additional information
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    let mut local_vec = vec![
        cformat!("<y>* Org: <m>({})</>", vec[0]),
    ];
    if vec.len() > 1 {
        local_vec.append(&mut vec[1..].iter().map(|s| s.to_string()).collect());
    }

    local_vec
}