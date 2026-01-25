use color_print::cformat;

/// vec: [>=2]
/// - 0 : pconf
/// - 1+: scopes
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    let mut local_vec: Vec<String> = Vec::new();
    local_vec.push(cformat!("<y>* Please check your token.</>"));
    local_vec.push(cformat!("  <g>» You must add the following scopes: </>"));
    vec[1..].iter().enumerate().for_each(|(i, s)| {
        local_vec.push(cformat!("    <#e3750e>{}. <m>{}</>", i + 1, s));
    });
    local_vec.push(cformat!("  <g>» Pconf : <m>{}</>", vec[0]));
    local_vec
}
