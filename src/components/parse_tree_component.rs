use bnf::Term;
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use stylance::import_style;

use crate::owned_tree::{OwnedParseTree, OwnedParseTreeNode};

import_style!(style, "parse_tree_component.module.scss");

#[component]
pub fn ParseTreeComponent(
    tree: OwnedParseTree,
    production_hues: Arc<HashMap<String, f32>>,
) -> impl IntoView {
    let lhs_name = match tree.lhs {
        Term::Terminal(name) => name,
        Term::Nonterminal(name) => name,
    };

    let get_color = |name: &str, production_hues: Arc<HashMap<String, f32>>| -> String {
        let hue = production_hues
            .get(&format!("<{}>", name))
            .unwrap_or(&0.0)
            .to_owned();
        format!("background-color: hsl({}deg, 16%, 50%)", hue)
    };

    view! {
        <div class=style::non_terminal_node style=get_color(&lhs_name, production_hues.clone())>
            <div class=style::terminal_lhs>
                {lhs_name.clone()}
            </div>
            <div>
                {tree.rhs.into_iter()
                    .map(|node| match node {
                        OwnedParseTreeNode::Terminal(terminal_value) => view! {
                            <div class=style::terminal_node>
                                {terminal_value}
                            </div>
                        }.into_any(),
                        OwnedParseTreeNode::Nonterminal(tree) => view! {
                            <ParseTreeComponent tree=tree production_hues=production_hues.clone() />
                        }.into_any()
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
