# CHANGELOG

## v0.3.1
### Fixes
- Fix `*=`, `/=` and `%=` for `Vec3`.

## v0.3.0
## New features
- Add `xy` and `yx` for `Vec2`.
- Add new trait `Scale` and method `scale` for `Vec2`.
- Add new traits `LargeType` and `NormalLimits` and range conversion methods to
  `Vec2`.
- Add trait `Two`.
- Add `to_polar` for `Vec2`.
- Add `Vec2::ZERO`.
- Add `cabs` to `Vec2`.
- Add `Vec3`.
- Add basic `Vec4`.

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
