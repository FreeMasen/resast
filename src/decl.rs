use crate::expr::{Expr, Lit};
use crate::pat::Pat;
use crate::VarKind;
use crate::{Class, Func, Ident};

/// The declaration of a variable, function, class, import or export
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum Decl<'a> {
    /// A variable declaration
    /// ```js
    /// var x, b;
    /// let y, a = 0;
    /// const q = 100
    /// ```
    Var(VarKind, Vec<VarDecl<'a>>),
    /// A function declaration
    /// ```js
    /// function thing() {}
    /// ```
    Func(Func<'a>),
    /// A class declaration
    /// ```js
    /// class Thing {}
    /// ```
    Class(Class<'a>),
    /// An import declaration
    /// ```js
    /// import * as moment from 'moment';
    /// import Thing, {thing} from 'stuff';
    /// ```
    Import(Box<ModImport<'a>>),
    /// An export declaration
    /// ```js
    /// export function thing() {}
    /// ```
    Export(Box<ModExport<'a>>),
}

/// The identifier and optional value of a variable declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct VarDecl<'a> {
    pub id: Pat<'a>,
    pub init: Option<Expr<'a>>,
}

/// A module declaration, This would only be available
/// in an ES Mod, it would be either an import or
/// export at the top level
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum ModDecl<'a> {
    Import(ModImport<'a>),
    Export(ModExport<'a>),
}

/// A declaration that imports exported
/// members of another module
///
/// ```js
/// import {Thing} from './stuff.js';
/// ```
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct ModImport<'a> {
    pub specifiers: Vec<ImportSpecifier<'a>>,
    pub source: Lit<'a>,
}

/// The name of the thing being imported
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum ImportSpecifier<'a> {
    /// A specifier in curly braces, this might
    /// have a local alias
    ///
    /// ```js
    /// import {Thing} from './stuff.js';
    /// import {People as Persons} from './places.js';
    /// ```
    Normal(NormalImportSpec<'a>),
    /// A specifier that has been exported with the
    /// default keyword, this should not be wrapped in
    /// curly braces.
    /// ```js
    /// import DefaultThing from './stuff/js';
    /// ```
    Default(Ident<'a>),
    /// Import all exported members from a module
    /// in a namespace.
    ///
    /// ```js
    /// import * as Moment from 'moment.js';
    /// ```
    Namespace(Ident<'a>),
}
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub struct NormalImportSpec<'a> {
    pub local: Ident<'a>,
    pub imported: Ident<'a>,
}

/// Something exported from this module
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum ModExport<'a> {
    /// ```js
    /// export default function() {};
    /// //or
    /// export default 1;
    /// ```
    Default(DefaultExportDecl<'a>),
    ///```js
    /// export {foo} from 'mod';
    /// //or
    /// export {foo as bar} from 'mod';
    /// //or
    /// export var foo = 1;
    /// //or
    /// export function bar() {
    /// }
    /// ```
    Named(NamedExportDecl<'a>),
    /// ```js
    /// export * from 'mod';
    /// ```
    All(Lit<'a>),
}

// pub struct NamedExportDecl<'a> {
//     decl: Option<Box<Decl<'a>>>,
//     specs: Vec<ExportSpecifier<'a>>,
//     source: Option<Cow<'a, str>>
// }
/// An export that has a name
/// ```js
/// export function thing() {}
/// export {stuff} from 'place';
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(all(feature = "serialization"), derive(Deserialize, Serialize))]
pub enum NamedExportDecl<'a> {
    Decl(Decl<'a>),
    Specifier(Vec<ExportSpecifier<'a>>, Option<Lit<'a>>),
}

/// A default export
/// ```js
/// export default class Thing {}
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub enum DefaultExportDecl<'a> {
    Decl(Decl<'a>),
    Expr(Expr<'a>),
}

/// The name of the thing being exported
/// this might include an alias
/// ```js
/// //no-alias
/// export {Thing} from 'place';
/// //aliased
/// export {Stuff as NewThing} from 'place'
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    all(feature = "serde", not(feature = "esprima")),
    derive(Deserialize, Serialize)
)]
#[cfg_attr(all(feature = "serde", feature = "esprima"), derive(Deserialize))]
pub struct ExportSpecifier<'a> {
    pub local: Ident<'a>,
    pub exported: Ident<'a>,
}
