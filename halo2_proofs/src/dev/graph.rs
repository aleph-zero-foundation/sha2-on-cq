use std::marker::PhantomData;
use ff::Field;
use tabbycat::{AttrList, Edge, GraphBuilder, GraphType, Identity, StmtList};
use halo2curves::pairing::MultiMillerLoop;

use crate::{
    circuit::Value,
    plonk::{
        Advice, Any, Assigned, Assignment, Challenge, Circuit, Column, ConstraintSystem, Error,
        Fixed, FloorPlanner, Instance, Selector,
    },
};
use crate::plonk::static_lookup::{StaticTable, StaticTableId};

pub mod layout;

/// Builds a dot graph string representing the given circuit.
///
/// The graph is built from calls to [`Layouter::namespace`] both within the circuit, and
/// inside the gadgets and chips that it uses.
///
/// [`Layouter::namespace`]: crate::circuit::Layouter#method.namespace
pub fn circuit_dot_graph<E: MultiMillerLoop, ConcreteCircuit: Circuit<E>>(
    circuit: &ConcreteCircuit,
) -> String {
    // Collect the graph details.
    let mut cs = ConstraintSystem::default();
    let config = ConcreteCircuit::configure(&mut cs);
    let mut graph = Graph::default();
    ConcreteCircuit::FloorPlanner::synthesize(&mut graph, circuit, config, cs.constants).unwrap();

    // Construct the node labels. We need to store these, because tabbycat operates on
    // string references, and we need those references to live long enough.
    let node_labels: Vec<_> = graph
        .nodes
        .into_iter()
        .map(|(name, gadget_name)| {
            if let Some(gadget_name) = gadget_name {
                format!("[{}] {}", gadget_name, name)
            } else {
                name
            }
        })
        .collect();

    // Construct the dot graph statements.
    let mut stmts = StmtList::new();
    for (id, label) in node_labels.iter().enumerate() {
        stmts = stmts.add_node(
            id.into(),
            None,
            Some(AttrList::new().add_pair(tabbycat::attributes::label(label))),
        );
    }
    for (parent, child) in graph.edges {
        stmts =
            stmts.add_edge(Edge::head_node(parent.into(), None).arrow_to_node(child.into(), None))
    }

    // Build the graph!
    GraphBuilder::default()
        .graph_type(GraphType::DiGraph)
        .strict(false)
        .id(Identity::id("circuit").unwrap())
        .stmts(stmts)
        .build()
        .unwrap()
        .to_string()
}

struct Graph<E> {
    /// Graph nodes in the namespace, structured as `(name, gadget_name)`.
    nodes: Vec<(String, Option<String>)>,

    /// Directed edges in the graph, as pairs of indices into `nodes`.
    edges: Vec<(usize, usize)>,

    /// The current namespace, as indices into `nodes`.
    current_namespace: Vec<usize>,

    _phantom: std::marker::PhantomData<E>,
}

impl<E> Default for Graph<E> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            current_namespace: vec![],
            _phantom: PhantomData::default(),
        }
    }
}

impl<E: MultiMillerLoop> Assignment<E::Scalar> for Graph<E> {
    type E = E;



    fn enter_region<NR, N>(&mut self, _: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        // Do nothing; we don't care about regions in this context.
    }

    fn exit_region(&mut self) {
        // Do nothing; we don't care about regions in this context.
    }

    fn register_static_table(&mut self, _id: StaticTableId<String>, _static_table: StaticTable<Self::E>) {}

    fn enable_selector<A, AR>(&mut self, _: A, _: &Selector, _: usize) -> Result<(), Error>
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        // Do nothing; we don't care about cells in this context.
        Ok(())
    }

    fn query_instance(&self, _: Column<Instance>, _: usize) -> Result<Value<E::Scalar>, Error> {
        Ok(Value::unknown())
    }

    fn assign_advice<'r, 'v>(&'r mut self, _column: Column<Advice>, _row: usize, _to: Value<Assigned<E::Scalar>>) -> Result<Value<&'v Assigned<E::Scalar>>, Error> {
        Ok(Value::unknown())
    }


    fn assign_fixed(&mut self, _column: Column<Fixed>, _row: usize, _to: Assigned<E::Scalar>) {
    }


    fn copy(
        &mut self,
        _: Column<Any>,
        _: usize,
        _: Column<Any>,
        _: usize,
    ) {}

    fn fill_from_row(
        &mut self,
        _: Column<Fixed>,
        _: usize,
        _: Value<Assigned<E::Scalar>>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn get_challenge(&self, _: Challenge) -> Value<E::Scalar> {
        Value::unknown()
    }

    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        // Store the new node.
        let new_node = self.nodes.len();
        self.nodes.push((name_fn().into(), None));

        // Create an edge from the parent, if any.
        if let Some(parent) = self.current_namespace.last() {
            self.edges.push((*parent, new_node));
        }

        // Push the new namespace.
        self.current_namespace.push(new_node);
    }

    fn pop_namespace(&mut self, gadget_name: Option<String>) {
        // Store the gadget name that was extracted, if any.
        let node = self
            .current_namespace
            .last()
            .expect("pop_namespace should never be called on the root");
        self.nodes[*node].1 = gadget_name;

        // Pop the namespace.
        self.current_namespace.pop();
    }
}
