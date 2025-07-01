use color_print::cformat;

/// vec: [>=2]
/// - 0 : description
/// - 1 : text
/// - 2+: additional information
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert!(vec.len() >= 2);
    let mut vect = vec![
        cformat!("<r>* Error:</> {}", vec[0]),
        cformat!("<m>* text:</> {}", vec[1]),
    ];
    
    for item in vec.iter().skip(2) {
        vect.push(item.to_string());
    };
    
    vect
}