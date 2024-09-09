use zen_engine::model::{
    DecisionContent, DecisionEdge, DecisionNode, DecisionNodeKind, DecisionTableContent,
    DecisionTableHitPolicy, DecisionTableInputField, DecisionTableOutputField, Expression,
    ExpressionNodeContent, FunctionNodeContent,
};

use crate::rule::Decision;
use std::collections::HashMap;

trait NodeBuilder {
    fn build(&self, decision: Decision) -> DecisionNode;
}

pub struct DecisionTableNodeBuilder;
impl NodeBuilder for DecisionTableNodeBuilder {
      fn build(&self, decision: Decision) -> DecisionNode {
        let Decision { id, rules, inputs, outputs, .. } = decision;
        let input_fields = make_fields(inputs, |f| DecisionTableInputField {
            id: f.clone(),
            name: f.clone(),
            field: Some(f.clone()),
        });
        let output_fields = make_fields(outputs, |f| DecisionTableOutputField {
            id: f.clone(),
            name: f.clone(),
            field: f.clone(),
        });
        let content = DecisionTableContent {
            hit_policy: DecisionTableHitPolicy::First,
            rules,
            inputs: input_fields,
            outputs: output_fields,
        };
        DecisionNode {
            id: id.clone(),
            name: id,
            kind: DecisionNodeKind::DecisionTableNode { content },
        }
    }
}

pub struct ExpressionNodeBuilder;
impl NodeBuilder for ExpressionNodeBuilder {
    fn build(&self, decision: Decision) -> DecisionNode {
        let Decision { id, expression, inputs, .. } = decision;
        let key = inputs.first().unwrap().to_string();
        let expression = Expression {
            id: key.clone(),
            key,
            value: expression.to_string(),
        };
        let content = ExpressionNodeContent {
            expressions: vec![expression],
        };
        DecisionNode {
            id: id.clone(),
            name: id,
            kind: DecisionNodeKind::ExpressionNode { content },
        }
    }
}

pub struct FunctionNodeBuilder;
impl NodeBuilder for FunctionNodeBuilder {
    fn build(&self, decision: Decision) -> DecisionNode {
        let Decision { id, function, .. } = decision;
        DecisionNode {
            id: id.clone(),
            name: id,
            kind: DecisionNodeKind::FunctionNode {
                content: FunctionNodeContent::Version1(function),
            },
        }
    }
}

// Node factory
pub struct NodeFactory {
    builders: HashMap<String, Box<dyn NodeBuilder>>,
}

impl NodeFactory {
    fn new() -> Self {
        let mut builders = HashMap::new();
        builders.insert("table".to_string(), Box::new(DecisionTableNodeBuilder) as Box<dyn NodeBuilder>);
        builders.insert("expression".to_string(), Box::new(ExpressionNodeBuilder) as Box<dyn NodeBuilder>);
        builders.insert("function".to_string(), Box::new(FunctionNodeBuilder) as Box<dyn NodeBuilder>);
        Self { builders }
    }

    fn create_node(&self, decision: Decision) -> DecisionNode {
        self.builders
            .get(&decision.kind)
            .expect(&format!("Unsupported decision kind: {}", decision.kind))
            .build(decision)
    }
}

fn make_fields<T, F>(fields: Vec<String>, field_creator: F) -> Vec<T>
where
    F: Fn(String) -> T,
{
    fields.into_iter().map(field_creator).collect()
}

pub struct EdgeBuilder;

impl EdgeBuilder {
    fn build_edges(flow: &[Decision]) -> Vec<DecisionEdge> {
        flow.iter().flat_map(|d| {
            d.sources.iter().map(|source| DecisionEdge {
                id: "".into(),
                source_id: source.clone(),
                target_id: d.id.clone(),
                source_handle: Some("".into()),
            }).chain(
                d.targets.iter().map(|target| DecisionEdge {
                    id: "".into(),
                    source_id: d.id.clone(),
                    target_id: target.clone(),
                    source_handle: Some("".into()),
                })
            )
        }).collect()
    }
}

pub struct DecisionGraphBuilder {
    node_factory: NodeFactory,
}

impl DecisionGraphBuilder {
    pub fn new() -> Self {
        Self {
            node_factory: NodeFactory::new(),
        }
    }

    pub fn build(&self, flow: Vec<Decision>) -> DecisionContent {
        let mut nodes = vec![
            DecisionNode {
                id: "request".to_string(),
                name: "request".to_string(),
                kind: DecisionNodeKind::InputNode,
            }
        ];

        nodes.extend(flow.iter().map(|d| self.node_factory.create_node(d.clone())));

        nodes.push(DecisionNode {
            id: "response".to_string(),
            name: "response".to_string(),
            kind: DecisionNodeKind::OutputNode,
        });

        let edges = EdgeBuilder::build_edges(&flow);

        DecisionContent { nodes, edges }
    }
}

pub async fn build(flow: Vec<Decision>) -> DecisionContent {
    DecisionGraphBuilder::new().build(flow)
}
