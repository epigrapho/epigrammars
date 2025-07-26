use bnf::Term;
use leptos::prelude::*;

use crate::owned_tree::{OwnedParseTree, OwnedParseTreeNode};

#[component]
pub fn ParseTreeComponent(tree: OwnedParseTree) -> impl IntoView {
    view! {
        {
            match tree.lhs {
                Term::Terminal(name) => view! { <li>"Terminal node: " {name}</li> },
                Term::Nonterminal(name) => view! { <li>"Nonterminal node: " {name}</li> }
            }
        }
        <ul>
            {tree.rhs.into_iter()
                .map(|node| match node {
                    OwnedParseTreeNode::Terminal(terminal_value) => view! { <li>Terminal: {terminal_value}</li> }.into_any(),
                    OwnedParseTreeNode::Nonterminal(tree) => view! { <ParseTreeComponent tree=tree /> }.into_any()
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}
