use std::str::FromStr;

use bnf::{Grammar, Term};
use leptos::prelude::*;
use owned_tree::{OwnedParseTree, OwnedParseTreeNode};

fn main() {
    leptos::mount::mount_to_body(App)
}

const EXAMPLE_GRAMMAR: &str = r#"
<number>           ::= <digit> | <digit> <number>
<digit>            ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' 
"#;

mod owned_tree;

#[component]
fn App() -> impl IntoView {
    let (grammar, set_grammar) = signal(EXAMPLE_GRAMMAR.to_string());
    let (input, set_input) = signal("1".to_string());

    let parsed_grammar = Memo::new(move |_| match Grammar::from_str(&grammar.get()) {
        Ok(grammar) => Ok(grammar),
        Err(bnf::Error::ParseError(e)) => Err(format!("Invalid grammar: {:}", e)),
        Err(e) => Err(format!("Failed to parse grammar, unknown error: {:#?}", e)),
    });

    let parsed_name = move || -> Result<_, String> {
        let grammar = parsed_grammar.get()?;
        let binding = input.get();
        let values = grammar.parse_input(&binding);
        let values = values.collect::<Vec<_>>();
        if let Some(value) = values.first() {
            Ok(OwnedParseTree::from_parse_tree(value))
        } else {
            Err(format!("No match"))
        }
    };

    view! {
        <textarea
            on:input:target=move |ev| {
                set_grammar.set(ev.target().value());
            }
            prop:value=grammar
        />

        <input
            on:input:target=move |ev| {
                set_input.set(ev.target().value());
            }
            prop:value=input
        />

        {
            move || match parsed_name() {
                Ok(value) => view! { <ul><ParseTreeComponent tree=value /></ul> }.into_any(),
                Err(e) => view! { <p>"Error" {e}</p> }.into_any()
            }
        }
    }
}

#[component]
fn ParseTreeComponent(tree: OwnedParseTree) -> impl IntoView {
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
