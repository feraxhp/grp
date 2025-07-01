use color_print::cformat;

/// vec: [>=2]
/// - 0 : path
/// - 1 : object (dir, file, etc.)
/// - 2+: additional information
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert!(vec.len() >= 2);
    let mut vect = vec![
        cformat!("<y>* The {} can not being found!</>", vec[1]),
        cformat!("  → {}", vec[0]),
    ];
    
    for item in vec.iter().skip(2) {
        vect.push(format!("  {}", item));
    };
    
    vect
}