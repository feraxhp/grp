use color_print::cformat;

/// vec: [>=2]
/// - 0 : object
/// - 1 : message
/// - 2+: additional information
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert!(vec.len() >= 2);
    let mut vect = vec![
        cformat!("<y>* The {} can not being found!</>", vec[0]),
        cformat!("  → <m>{}</>", vec[1]),
    ];
    
    for item in vec.iter().skip(2) {
        vect.push(format!("  {}", item));
    };
    
    vect
}