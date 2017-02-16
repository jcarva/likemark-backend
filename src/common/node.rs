use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::FromEntity;
use common::entity::Entity;
use std::cmp::Ordering;

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    name: String,
    url: String,
    fn_type: FnType,
    link: Vec<Link>,
    nodes: Vec<Node>,
}

impl Node {
    fn set_children(&self, children: &[Entity]) {

    }
}

impl FromEntity for Node {
    fn from_entity(entity: Entity) -> Self {
        Node {
            id: entity.id,
            name: entity.name,
            url: entity.url,
            fn_type: entity.fn_type,
            link: Vec::new(),
            nodes: Vec::new()
        }
    }

    //TODO entity matcher for nodes
    fn from_entities(mut entites: Vec<Entity>) -> Vec<Self> {
        let mut node_vec: Vec<Node> = Vec::new();
        let mut link_vec: Vec<Link> = Vec::new();

        //let base_length = 2;
        entites.sort_by(|e1, e2| {
            if e1.tree_id.len() < e2.tree_id.len() {
                Ordering::Less
            } else if e1.tree_id.len() > e2.tree_id.len() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        //remplir les tableau node_vec et link_vec
        for entity in entites{
            if entity.struct_type == StructType::Node{
                node_vec.push(Node::from_entity(entity));
            }
            else if entity.struct_type == StructType::Link{
                link_vec.push(Link::from_entity(entity));
            }
        }

        //ajoute les liens associer au noeud correspondant
        // manque l'attribut link.tree_id et manque condition du prefix de tree_id
        for node in node_vec{
            for link in link_vec{
                if link.tree_id.len() == entity.tree_id.len() + 1 {
                    node.link.push(link);
                }
            }
        }

        println!("{:?}", node_vec);
        node_vec
    }
}
