use crate::object::Object;

/// A morphism between objects in a category.
///
/// In category theory, morphisms are maps between objects that generalize the notion
/// of functions between sets. Each morphism has:
/// - A domain (source object)
/// - A codomain (target object)
/// - A mapping operation that transforms elements from domain to codomain
///
/// Morphisms must preserve the category's structure and are composable when
/// the domain of one matches the codomain of another.
pub trait Morphism {
    type Domain: Object;
    type Codomain: Object;
    fn domain(&self) -> &Self::Domain;
    fn codomain(&self) -> &Self::Codomain;
    fn map(&self, domain: &Self::Domain) -> Self::Codomain;
}

/// Compares two morphisms for equality by checking domain, codomain, and behavior.
///
/// Two morphisms are considered equal if:
/// 1. They have the same domain
/// 2. They have the same codomain
/// 3. They transform their domain to the same result
pub fn check_eq_morphisms<A: Object, B: Object>(
    first: &dyn Morphism<Domain = A, Codomain = B>,
    second: &dyn Morphism<Domain = A, Codomain = B>,
) -> bool {
    if first.domain() == second.domain()
        && first.codomain() == second.codomain()
        && first.map(first.domain()) == second.map(second.domain())
    {
        return true;
    }
    false
}

/// Composes two morphisms f: A → B and g: B → C to produce g∘f: A → C.
///
/// In category theory, composition is a fundamental operation that combines
/// two compatible morphisms (where the codomain of the first equals the domain of the second)
/// to create a new morphism.
pub fn compose<A: Object, B: Object, C: Object>(
    domain: &A,
    first: &dyn Morphism<Domain = A, Codomain = B>,
    second: &dyn Morphism<Domain = B, Codomain = C>,
) -> C {
    second.map(&first.map(domain))
}

/// A collection of morphisms between two specific objects.
///
/// In category theory, Hom(A,B) represents all possible morphisms from object A to object B.
pub type HomSet<A, B> = Vec<Box<dyn Morphism<Domain = A, Codomain = B>>>;
