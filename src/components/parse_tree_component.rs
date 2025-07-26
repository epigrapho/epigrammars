use std::collections::HashMap;

use bnf::Term;
use leptos::prelude::*;
use stylance::import_style;

use crate::owned_tree::{OwnedParseTree, OwnedParseTreeNode};

import_style!(style, "parse_tree_component.module.scss");

#[component]
pub fn ParseTreeComponent(
    tree: OwnedParseTree,
    production_hues: ArcMemo<HashMap<String, f32>>,
) -> impl IntoView {
    let lhs_name = match tree.lhs {
        Term::Terminal(name) => name,
        Term::Nonterminal(name) => name,
    };

    let hue = production_hues
        .get()
        .get(&format!("<{}>", lhs_name))
        .unwrap_or(&0.0)
        .to_owned();
    let color = format!("background-color: hsl({}deg, 16%, 50%)", hue);

    view! {
        <div class=style::non_terminal_node style=color>
            <div class=style::terminal_lhs>
                {lhs_name}
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
