use color_print::cformat;

/// vec: [>=2]
/// - 0 : platform
/// - 1 : action
/// - 2+: additional info
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    debug_assert!(vec.len() >= 2);
    let mut vect = vec![
        cformat!("<y>* <m>{}</m> does not suppot <r,i>{}!</>", vec[0], vec[1]),
    ];
    
    for item in vec.iter().skip(2) {
        vect.push(format!("  {}", item.to_string()));
    };
    
    vect
}