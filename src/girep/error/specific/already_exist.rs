use color_print::cformat;

use crate::repo;

const SPECIAL: [&str; 2] = ["Repo", "Org"];

/// vec: [>=2]
/// - 0 : type ("Repo", "Orgs", etc.)
/// - 1 : name ("owner", "name", etc.)
/// - 2 : additional info (e.g., "repo_name", etc.)
pub(crate) fn content(vec: Vec<String>) -> Vec<String> {
    assert!(vec.len() >= 2, "content requires at least 2 elements in the vector");
    let type_ = vec[0].clone();
    let mut vect = vec![
        cformat!("<y>* {} already exists!</>", &type_),
        match type_.as_str() {
            "Repo" => cformat!("  <g>» Repo: {}", repo!(vec[1], vec[2])),
            "Org" => cformat!("  <g>» Org: <m>{}</>", vec[1]),
            r#type => cformat!("  <g>» {}: <m>{}</>", r#type, vec[1]),
        },
    ];
    
    if !SPECIAL.contains(&type_.as_str()) {
        vect.extend(vec.iter().skip(2).into_iter().map(|d| d.to_string()));
    };
    
    vect
}
