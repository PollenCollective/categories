//! # Categories
//!
//! Categories over generic objects with power object management.
//!
//! This module provides traits and data structures for categorical constructions in mathematics.
//! A category consists of objects and morphisms (maps between objects) that satisfy certain
//! properties:
//! - Every object has an identity morphism
//! - Morphisms can be composed when their domain/codomain match
//! - Composition is associative
//!
//! The implementation supports:
//! - Generic objects and morphisms with type safety
//! - Power object management (products, coproducts, exponentials)
//! - Both lazy construction and builder pattern approaches
//!
//! ## Mathematical Background
//!
//! ### Core Categorical Concepts
//!
//! - **Objects**: Abstract entities in a category (can represent sets, groups, topological spaces,
//!   etc.)
//! - **Morphisms**: Maps between objects (generalizations of functions between sets)
//! - **HomSet**: Collection of all morphisms between two specific objects A and B, denoted Hom(A,B)
//!
//! ### Power Objects
//!
//! Power objects represent constructions between objects:
//!
//! - **Product**: A × B with projections π₁: A×B → A and π₂: A×B → B satisfying the universal
//!   property
//! - **Coproduct**: A + B with injections i₁: A → A+B and i₂: B → A+B satisfying the universal
//!   property
//! - **Exponential**: Bᴬ representing "all morphisms from A to B" with evaluation map ev: Bᴬ×A → B
//!
//! ### Special Morphisms
//!
//! - **Monic**: A morphism f is monic if it's left-cancellable: f∘g₁ = f∘g₂ implies g₁ = g₂
//! - **Terminal Object**: An object T where for every object A, there exists exactly one morphism A
//!   → T

pub mod categories;
pub mod morphism;
pub mod object;
