use orbweaver as ow;
use wasm_bindgen::prelude::{wasm_bindgen, Closure, JsValue};

#[wasm_bindgen]
struct Node(ow::Node<JsValue>);

#[wasm_bindgen]
struct DirectedGraph(ow::DirectedGraph<JsValue>);

#[wasm_bindgen]
struct Edge(ow::Edge);

#[wasm_bindgen]
impl DirectedGraph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DirectedGraph(ow::DirectedGraph::new())
    }

    pub fn add_node(&mut self, id: &str, data: JsValue) -> Result<(), wasm_bindgen::JsError> {
        self.0.add_node(id, data)?;
        Ok(())
    }

    pub fn add_edge(&mut self, from: &str, to: &str) -> Result<(), wasm_bindgen::JsError> {
        self.0.add_edge(from, to)?;
        Ok(())
    }

    pub fn add_path(&mut self, path: Vec<String>) -> Result<(), wasm_bindgen::JsError> {
        self.0.add_path(&path)?;
        Ok(())
    }

    pub fn get_edge(&self, from: &str, to: &str) -> Option<Edge> {
        self.0.get_edge(from, to).map(Edge)
    }

    pub fn get_incoming_edges(&self, node: &str) -> Vec<Edge> {
        self.0
            .get_incoming_edges(node)
            .into_iter()
            .map(Edge)
            .collect()
    }

    pub fn nodes(&self) -> Vec<String> {
        self.0.nodes().map(|node| node.id().to_string()).collect()
    }

    pub fn clone(&self) -> Self {
        DirectedGraph(self.0.clone())
    }

    pub fn find_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        self.0
            .find_path(from, to)
            .map(|path| path.into_iter().map(String::from).collect())
    }

    pub fn children(&self, node: &str) -> Vec<String> {
        self.0
            .children(node)
            .into_iter()
            .map(String::from)
            .collect()
    }

    pub fn parents(&self, node: &str) -> Vec<String> {
        self.0.parents(node).into_iter().map(String::from).collect()
    }

    pub fn clear_edges(&mut self) {
        self.0.clear_edges();
    }
}
