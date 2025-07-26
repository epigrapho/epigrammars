use bnf::Term;
use leptos::prelude::*;
use stylance::import_style;

use crate::owned_tree::{OwnedParseTree, OwnedParseTreeNode};

import_style!(style, "parse_tree_component.module.scss");

#[component]
pub fn ParseTreeComponent(tree: OwnedParseTree) -> impl IntoView {
    view! {
        <div class=style::non_terminal_node>
            {
                match tree.lhs {
                    Term::Terminal(name) => view! {
                        <div class=style::terminal_lhs>
                            {name}
                        </div>
                    },
                    Term::Nonterminal(name) => view! {
                        <div class=style::nonterminal_lhs>
                            {name}
                        </div>
                    }
                }
            }
            <div>
                {tree.rhs.into_iter()
                    .map(|node| match node {
                        OwnedParseTreeNode::Terminal(terminal_value) => view! {
                            <div class=style::terminal_node>
                                {terminal_value}
                            </div>
                        }.into_any(),
                        OwnedParseTreeNode::Nonterminal(tree) => view! {
                            <ParseTreeComponent tree=tree />
                        }.into_any()
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
