# CHANGELOG

## future
### Breaking changes
- `Vec2::in_range` has been renamed to `Vec2::contains`.
- `Vec2::contains` has been renamed to `Vec2::size_contains`.
- Methods `map`, `contains`, `cast` and `convert` on `Vec2` have been moved to
  trait implementation.
- Methods `map`, `cast` and `convert` on `Vec3` have been moved to trait
  implementation.
- Method `rect_center` on `Vec4` has been moved to trait implementation.

### New features
- Range operations `join`, `valid_range_or_empty`, `intersect`, `intersects`,
  `touches`, `join_gap`, `sub_range`, `sub_range_gap`, `sym_sub_range` for
  `Vec2`.
- New traits `TwoComponent`, `MapExt` and `RangeExt` that can add methods to
  types outside this crate. They are notably implemented to `Range<T>`.
- New trait `RectExt`.
- New type `Rect`.
- New type `Padding`.
- Implement arithmetic on `Vec4`.
- Add checked and saturating arithmetic.

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
