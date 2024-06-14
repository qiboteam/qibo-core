# Qibo-core

The scope of the project is to represent the primitives for quantum execution, in order
to share a minimal common layer among the various execution actors, i.e. the possible
high-level APIs and various execution backends.

Moreover, these primitives will be supplemented with a backend protocol, whose basic
ingredients are provided by the distributed package as well, to standardize and ease the
implementation of the various parts of the ecosystem.

## Similarity with other quantum representations

There are other projects with a similar scope, i.e. provide a backend-agnostic
intermediate representation of a quantum execution.
A couple of notable examples are OpenQASM and QIR.

However, this project is pretty much distinct from those two, as it is explicitly tuned
for a given ecosystem, and with the intention to provide a more opinionated execution
mechanism, implementing some of the structures involved, beyond the basic representation
as a language.
Conversely, it is possibly also more restricted as a language itself, since the
aim is only at executing the quantum primitives, staying as close as possible to what
can be realized even on quantum hardware, possibly at the lowest level.

## Relation with Qibo

With respect to the main package, `qibo-core` has no purpose to provide a user-friendly
API, nor any other high-level feature.
Instead, the goal is to make it friendly for frontend and backend implementors,
deduplicating most of the shared elements.

High-level features will be maintained in Qibo's main package, which is no way going to
be phased out by `qibo-core`.
Instead, `qibo-core` will be used internally in `qibo` in place of its current
structures, mediating the communications with the backends.

## Content

- [structure](./structure.md)
- [backends](./backends.md)
- [APIs](./apis.md)
