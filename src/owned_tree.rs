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
}

pub struct OwnedParseTree {
    pub lhs: Term,
    pub rhs: Vec<OwnedParseTreeNode>,
}

impl OwnedParseTree {
    pub fn from_parse_tree(ref_tree: &ParseTree) -> Self {
        OwnedParseTree {
            lhs: ref_tree.lhs.to_owned(),
            rhs: ref_tree
                .rhs_iter()
                .map(OwnedParseTreeNode::from_parse_tree_node)
                .collect(),
        }
    }
}
