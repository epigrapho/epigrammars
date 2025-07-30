use crate::{components::parse_tree_component::ParseTreeComponent, owned_tree::OwnedParseTree};
use bnf::Grammar;
use leptos::{logging::log, prelude::*};
use std::sync::Arc;
use std::{collections::HashMap, str::FromStr};
use stylance::import_style;

const EXAMPLE_GRAMMAR: &str = r#"<sentence>   ::= <subject> <space> <verb> <space> <complement> '.'
 
<noun_group> ::= <article> <space> <noun> | <plural_noun>
<noun>       ::= 'mouse' | 'dog' | 'cat'
<plural_noun> ::= <noun> 's'
<verb>       ::= 'eats' | 'search for' | 'look after' | 'is afraid of'
<subject>    ::= <noun_group>
<complement> ::= <noun_group>
<space>      ::= ' '
<article>    ::= 'the' | 'a'
"#;

import_style!(style, "app.module.scss");

#[component]
pub fn App() -> impl IntoView {
    let (grammar, set_grammar) = signal(EXAMPLE_GRAMMAR.to_string());
    let (input, set_input) = signal("the mouse is afraid of cats.".to_string());
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

    let colors = Memo::new(move |_| {
        let mut hue_by_name = HashMap::new();
        let len = outputs.get().len() as f32;
        for (i, output) in outputs.get().into_iter().enumerate() {
            hue_by_name.insert(output, 360.0 * (i as f32) / len);
        }
        Arc::new(hue_by_name)
    });

    let parsed_name = move || -> Result<_, String> {
        let grammar = grammar_with_target.get()?;
        let binding = input.get();
        let values = grammar.parse_input(&binding);
        let values = values
            .map(|r| OwnedParseTree::from_parse_tree(&r))
            .collect::<Vec<_>>();
        Ok(values)
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

            <div class=style::production_buttons>
                {
                    move || outputs
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(i, production)| {
                            let hue = 360.0 * (i as f32) / (outputs.get().len() as f32);
                            let active = if i == selected_production.get() { "font-weight: bold" } else { "" };
                            let style = format!("background-color: hsl({}deg, 16%, 50%);{}", hue, active);
                            view! {
                            <div>
                                <button on:click:target=move |_| set_selected_production.set(i)
                                        style=style>
                                    { production }
                                </button>
                            </div>
                        }})
                        .collect::<Vec<_>>()
                }
            </div>

            {
                move || match parsed_name() {
                    Ok(productions) if !productions.is_empty() => productions
                        .into_iter()
                        .enumerate()
                        .map(|(i, production)| view! {
                            <h2>Match #{i}</h2>
                            <div class=style::production_container>
                                <ParseTreeComponent tree=production production_hues={colors.get()} />
                            </div>
                        }.into_any())
                        .collect::<Vec<_>>(),
                    Ok(_) => vec![ view! {
                        <div class=style::alert_error>
                            "No match"
                        </div>
                    }.into_any() ],
                    Err(e) => vec![ view! {
                        <div class=style::alert_error>
                            <pre>
                                {e.replace("\\n", "\n")}
                            </pre>
                        </div>
                    }.into_any() ]
                }
            }
        </div>
    }
}
