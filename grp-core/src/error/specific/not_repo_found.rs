use color_print::cformat;
use crate::repo;

/// vec: [==2]
/// - 0 : owner
/// - 1 : repo
/// - 2*: platform
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    assert!(vec.len() == 2 || vec.len() == 3);
    let mut vect = vec![cformat!("  <g>» Repo: <m>({})</>", repo!(vec[0], vec[1]))];
    
    if vec.len() == 3 {
        vect.push(cformat!("  <g>» Platform: <y>({})</>", vec[2]));
    }
    
    vect
}