mod char_data;
pub mod comment;
pub mod document;
mod element;
mod element_factory;
mod elements;
pub mod node;
pub mod text;
mod token_list;

pub use element_factory::create_element;
