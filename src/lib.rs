use orbweaver::prelude as ow;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsObject, JsValue};

#[wasm_bindgen]
pub fn panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct DirectedGraphBuilder(ow::DirectedGraphBuilder);

#[wasm_bindgen]
impl DirectedGraphBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DirectedGraphBuilder(ow::DirectedGraphBuilder::new())
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.0.add_edge(from, to);
    }

    pub fn add_path(&mut self, path: Vec<String>) {
        self.0.add_path(path);
    }

    pub fn build_directed(&mut self) -> DirectedGraph {
        let mut new_builder = ow::DirectedGraphBuilder::new();
        std::mem::swap(&mut self.0, &mut new_builder);
        DirectedGraph(new_builder.build_directed())
    }

    pub fn build_acyclic(&mut self) -> Result<DirectedAcyclicGraph, JsError> {
        let mut new_builder = ow::DirectedGraphBuilder::new();
        std::mem::swap(&mut self.0, &mut new_builder);
        Ok(DirectedAcyclicGraph(new_builder.build_acyclic()?))
    }
}

impl Default for DirectedGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct DirectedGraph(ow::DirectedGraph);

#[wasm_bindgen]
pub struct DirectedAcyclicGraph(ow::DirectedAcyclicGraph);

#[wasm_bindgen]
impl DirectedGraph {
    pub fn from_bin(val: &[u8]) -> Result<DirectedGraph, JsError> {
        Ok(DirectedGraph(ow::DirectedGraph::from_binary(val)?))
    }

    pub fn to_bin(&self) -> Result<Vec<u8>, JsError> {
        let mut buffer = Vec::new();
        self.0.to_binary(&mut buffer)?;
        Ok(buffer)
    }

    pub fn deep_clone(&self) -> Self {
        DirectedGraph(self.0.clone())
    }

    pub fn find_path(&self, from: &str, to: &str) -> Result<Vec<String>, JsError> {
        Ok(self
            .0
            .find_path(from, to)?
            .into_iter()
            .map(String::from)
            .collect())
    }

    pub fn children(&self, nodes: Vec<String>) -> Vec<String> {
        self.0
            .children(nodes)
            .map(|children| children.iter().map(String::from).collect())
            .unwrap_or_default()
    }

    pub fn parents(&self, nodes: Vec<String>) -> Vec<String> {
        self.0
            .parents(nodes)
            .map(|parents| parents.iter().map(String::from).collect())
            .unwrap_or_default()
    }

    pub fn has_children(&self, nodes: Vec<String>) -> Result<Vec<u8>, JsError> {
        Ok(self
            .0
            .has_children(nodes)?
            .into_iter()
            .map(|x| x as u8)
            .collect())
    }

    pub fn least_common_parents(&self, selected: Vec<String>) -> Result<Vec<String>, JsError> {
        Ok(self
            .0
            .least_common_parents(&selected)?
            .into_iter()
            .map(String::from)
            .collect())
    }

    pub fn get_all_leaves(&self) -> Vec<String> {
        self.0
            .get_all_leaves()
            .into_iter()
            .map(String::from)
            .collect()
    }

    pub fn get_leaves_under(&self, node_ids: Vec<String>) -> Result<Vec<String>, JsError> {
        Ok(self
            .0
            .get_leaves_under(node_ids.as_slice())?
            .into_iter()
            .map(String::from)
            .collect())
    }

    pub fn nodes(&self) -> Vec<String> {
        self.0.nodes().into_iter().map(String::from).collect()
    }

    pub fn length(&self) -> u64 {
        self.0.len() as u64
    }

    pub fn find_path_one_to_many(&self, from: &str, to: Vec<String>) -> Result<JsValue, JsError> {
        let value = self.0.find_path_one_to_many(from, to)?;
        Ok(serde_wasm_bindgen::to_value(&value)?)
    }
}

#[wasm_bindgen]
impl DirectedAcyclicGraph {
    pub fn from_bin(val: &[u8]) -> Result<DirectedAcyclicGraph, JsError> {
        Ok(DirectedAcyclicGraph(ow::DirectedAcyclicGraph::from_binary(
            val,
        )?))
    }

    pub fn to_bin(&self) -> Result<Vec<u8>, JsError> {
        let mut buffer = Vec::new();
        self.0.to_binary(&mut buffer)?;
        Ok(buffer)
    }
    pub fn into_directed_graph(&self) -> DirectedGraph {
        DirectedGraph(self.0.clone().into_inner())
    }

    pub fn deep_clone(&self) -> Self {
        DirectedAcyclicGraph(self.0.clone())
    }

    pub fn find_path(&self, from: &str, to: &str) -> Result<Vec<String>, JsError> {
        Ok(self
            .0
            .find_path(from, to)?
            .into_iter()
            .map(String::from)
            .collect())
    }

    pub fn children(&self, nodes: Vec<String>) -> Vec<String> {
        self.0
            .children(nodes)
            .map(|children| children.iter().map(String::from).collect())
            .unwrap_or_default()
    }

    pub fn parents(&self, nodes: Vec<String>) -> Vec<String> {
        self.0
            .parents(nodes)
            .map(|parents| parents.iter().map(String::from).collect())
            .unwrap_or_default()
    }

    pub fn has_children(&self, nodes: Vec<String>) -> Result<Vec<u8>, JsError> {
        Ok(self
            .0
            .has_children(nodes)?
            .into_iter()
            .map(|x| x as u8)
            .collect())
    }

    pub fn least_common_parents(&self, selected: Vec<String>) -> Result<Vec<String>, JsError> {
        Ok(self
            .0
            .least_common_parents(selected)?
            .into_iter()
            .map(String::from)
            .collect())
    }

    pub fn get_all_leaves(&self) -> Vec<String> {
        self.0
            .get_all_leaves()
            .into_iter()
            .map(String::from)
            .collect()
    }

    pub fn get_leaves_under(&self, node_ids: Vec<String>) -> Result<Vec<String>, JsError> {
        Ok(self
            .0
            .get_leaves_under(node_ids.as_slice())?
            .into_iter()
            .map(String::from)
            .collect())
    }

    pub fn nodes(&self) -> Vec<String> {
        self.0.nodes().into_iter().map(String::from).collect()
    }

    pub fn length(&self) -> u64 {
        self.0.len() as u64
    }

    pub fn find_path_one_to_many(&self, from: &str, to: Vec<String>) -> Result<JsValue, JsError> {
        let value = self.0.find_path_one_to_many(from, to)?;
        Ok(serde_wasm_bindgen::to_value(&value)?)
    }
}
