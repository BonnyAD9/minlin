# CHANGELOG

## future
## New features
- Add `xy` and `yx` for `Vec2`.
- Add new trait `Scale` and method `scale` for `Vec2`.
- Add `to_polar` for `Vec2`.
- Add `Vec3`.

### Changes
- Normalization functions for `Vec2` are now more generic.

### Breaking changes
- `Vec2::from_polar` now takes the arguments in different order.

## v0.2.0
- Add new methods for `Vec2`: `pos_of_idx`, `angle`, `normalized`,
  `norzmalize`, `to`, `from_polar`, `idx_of_pos` and `contains`.
- Add new traits: `ContainingFloat`, `Goniometric`, `Float`, `IntoFloat`,
  `One` and `Zero`.
- Add `Vec2Range`.
- Implement `Cast` for float types.
- Implement `Neg` for `Vec2`.

## v0.1.0
- Add `Vec2`.
