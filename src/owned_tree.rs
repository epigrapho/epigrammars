use bnf::{ParseTree, ParseTreeNode, Term};

pub enum OwnedParseTreeNode {
    Terminal(String),
    Nonterminal(OwnedParseTree),
}

impl OwnedParseTreeNode {
    pub fn from_parse_tree_node(node: &ParseTreeNode) -> Self {
        match node {
            ParseTreeNode::Terminal(value) => OwnedParseTreeNode::Terminal(value.to_string()),
            ParseTreeNode::Nonterminal(tree) => {
                OwnedParseTreeNode::Nonterminal(OwnedParseTree::from_parse_tree(tree))
            }
        }
    }

    pub fn get_weight(&self) -> u32 {
        if let OwnedParseTreeNode::Nonterminal(tree) = self {
            tree.weight
        } else {
            1
        }
    }
}

pub struct OwnedParseTree {
    pub weight: u32,
    pub lhs: Term,
    pub rhs: Vec<OwnedParseTreeNode>,
}

impl OwnedParseTree {
    pub fn from_parse_tree(ref_tree: &ParseTree) -> Self {
        let rhs: Vec<_> = ref_tree
            .rhs_iter()
            .map(OwnedParseTreeNode::from_parse_tree_node)
            .collect();
        let weight = rhs.iter().map(|x| x.get_weight()).sum();

        OwnedParseTree {
            lhs: ref_tree.lhs.to_owned(),
            rhs,
            weight,
        }
    }
}
