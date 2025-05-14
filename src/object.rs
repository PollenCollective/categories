use crate::morphism::Morphism;
use std::{fmt::Debug, hash::Hash};

/// An object in a category.
///
/// In category theory, objects are abstract entities. They could represent anything from
/// sets to groups, topological spaces, or even other categories.
///
/// Objects must be clonable, comparable, and debuggable to support categorical operations.
pub trait Object: Clone + PartialEq + Debug {}

/// Types of power objects that can be generated in a category.
///
/// Power objects represent different ways to construct new objects from existing ones:
/// - Product: Combines two objects with projections (like cartesian product for sets)
/// - Coproduct: Represents disjoint union with injections (like disjoint union for sets)
/// - Exponential: Represents "morphism objects" (like function spaces for sets)
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PowerObjectType {
    /// Product of objects at indices i and j (A×B with projections)
    Product(usize, usize),
    /// Coproduct of objects at indices i and j (A+B with injections)
    Coproduct(usize, usize),
    /// Exponential object representing "all morphisms from i to j" (Bᴬ)
    Exponential(usize, usize),
}

/// Generates power objects of a specific type for a category.
///
/// This trait allows categories to construct the standard categorical power objects:
/// - Products (A×B) with their projection morphisms
/// - Coproducts (A+B) with their injection morphisms
/// - Exponentials (Bᴬ) with their evaluation morphisms
pub trait PowerObjectGenerator<O: Object> {
    fn generate_power_object(
        &self,
        power_type: &PowerObjectType,
        objects: &[O],
    ) -> (O, Vec<Box<dyn Morphism<Domain = O, Codomain = O>>>);
}
