use eframe::egui::{Ui, Response};

pub struct TreeViewResponse<Leaf>{
    pub response: Response,
    pub clicked_leaf: Option<Leaf>
}
 
pub trait TreeViewNode<Leaf>
where
    Self: Sized,
    Leaf: TreeViewLeaf
{

    fn title(&self) -> String;
    fn children(&self) -> Vec<Self>;
    fn leaves(&self) -> Vec<Leaf>;
}

pub trait TreeViewLeaf
{
    fn title(&self) -> String;
}

pub fn tree_view_show<L, N>(ui: &mut Ui, node: N) -> TreeViewResponse<L>
    where L: TreeViewLeaf,
        N: TreeViewNode< L>,
    {
    let collapsing_response = ui.collapsing(node.title(), |ui| {
        let mut clicked_leaf = None;
        for node in node.children() {
            let inner_clicked = tree_view_show(ui, node).clicked_leaf;
            clicked_leaf = clicked_leaf.or(inner_clicked);
        }
        for leaf in node.leaves(){
            if ui.small_button(leaf.title()).clicked() {
                clicked_leaf = Some(leaf);
            }
        }
        clicked_leaf
    });

    TreeViewResponse{
        response: collapsing_response.header_response,
        clicked_leaf: collapsing_response.body_returned.flatten()
    }
}