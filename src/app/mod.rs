use std::str::FromStr;

use crate::{components::parse_tree_component::ParseTreeComponent, owned_tree::OwnedParseTree};
use bnf::Grammar;
use leptos::prelude::*;
use stylance::import_style;

const EXAMPLE_GRAMMAR: &str = r#"
<number>           ::= <digit> | <digit> <number>
<digit>            ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' 
"#;

import_style!(style, "app.module.scss");

#[component]
pub fn App() -> impl IntoView {
    let (grammar, set_grammar) = signal(EXAMPLE_GRAMMAR.to_string());
    let (input, set_input) = signal("1".to_string());
    let (selected_production, set_selected_production) = signal(0);

    let parsed_grammar = Memo::new(move |_| match Grammar::from_str(&grammar.get()) {
        Ok(grammar) => Ok(grammar),
        Err(bnf::Error::ParseError(e)) => Err(format!("Invalid grammar: {:}", e)),
        Err(e) => Err(format!("Failed to parse grammar, unknown error: {:#?}", e)),
    });

    let grammar_with_target = Memo::new(move |_| -> Result<Grammar, String> {
        let grammar_copy = parsed_grammar.get()?;
        let mut productions = grammar_copy.productions_iter().collect::<Vec<_>>();
        if selected_production.get() > productions.len() {
            Ok(grammar_copy)
        } else {
            let mut new_grammar = Grammar::new();
            productions.swap(0, selected_production.get());
            productions
                .into_iter()
                .for_each(|production| new_grammar.add_production(production.clone()));
            Ok(new_grammar)
        }
    });

    let outputs = Memo::new(move |_| {
        parsed_grammar
            .get()
            .map(|grammar| {
                grammar
                    .productions_iter()
                    .map(|production| production.lhs.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or(vec![])
    });

    let parsed_name = move || -> Result<_, String> {
        let grammar = grammar_with_target.get()?;
        let binding = input.get();
        let values = grammar.parse_input(&binding);
        let values = values
            .map(|r| OwnedParseTree::from_parse_tree(&r))
            .collect::<Vec<_>>();

        if values.is_empty() {
            Err(format!("No match"))
        } else {
            Ok(values)
        }
    };

    view! {
        <div class=style::header>
            <div>
                EpiGramma<span>rs</span>
            </div>
        </div>

        <div class=style::main>
            <div class=style::grammar_input>
                <h2>
                    Grammar
                </h2>
                <textarea
                    on:input:target=move |ev| {
                        set_grammar.set(ev.target().value());
                    }
                    prop:value=grammar
                />
            </div>

            <div class=style::input_input>
                <h2>
                    Input
                </h2>
                <textarea
                    on:input:target=move |ev| {
                        set_input.set(ev.target().value());
                    }
                    prop:value=input
                />
            </div>

            {
                move || outputs
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, production)| view! {
                        <div>
                            <input type="radio"
                                   id={production.clone()}
                                   name="production"
                                   value={i}
                                   prop:checked=move || i == selected_production.get()
                                   on:input:target=move |_| set_selected_production.set(i) />
                            <label for={production}>{production.clone()}</label>
                        </div>
                    })
                    .collect::<Vec<_>>()
            }

            {
                move || match parsed_name() {
                    Ok(productions) => productions
                        .into_iter()
                        .map(|production| view! {
                            <h2>Production</h2>
                            <div class=style::production_container>
                                <ParseTreeComponent tree=production />
                            </div>
                        }.into_any())
                        .collect::<Vec<_>>(),
                    Err(e) => vec![ view! { <p>"Error" {e}</p> }.into_any() ]
                }
            }
        </div>
    }
}
