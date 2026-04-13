# Copilot Instructions for Train Driver Firmware

This repository contains embedded Rust firmware for STM32F103 using `no_std`, `cortex-m-rt`, `defmt`, and `stm32f1xx-hal`.

## Project Scope

- Keep APIs small and explicit; avoid broad refactors unless requested.
- Focus on deterministic, low-level logic suitable for embedded systems.
- Look for ways to make things generic and modular

## Embedded Rust Rules

- Keep firmware `no_std` and `no_main` compatible.
- Avoid heap allocations and std-only APIs.
- Prefer deterministic, low-overhead logic suitable for interrupt-driven systems.
- Use fixed-size integers with explicit conversions to avoid implicit narrowing.

## HAL and Peripheral Conventions

- Use `stm32f1xx-hal` idioms for peripheral setup and pin ownership.
- Be explicit about timer channel/pin mappings to avoid type inference ambiguity.
- Current PWM convention in this repo:
  - `TIM3 CH1/CH2` for LED PWM on `PA6/PA7`
  - `TIM3 CH3/CH4` for motor PWM on `PB0/PB1`
- Current ADC convention:
  - `PA0` for channel 0
  - `PA1` for channel 1
- Do Not changes pins or timers without explicit instructions to do so.

## Logging and Errors

- Use `defmt` logging (`info!`, etc.) for runtime diagnostics.
- Any type logged with `defmt` should derive or implement `defmt::Format`.
- Prefer typed error enums over strings for device and conversion failures.

## Types and Validation

- Use validated newtypes (for example percentage-like values) instead of loose aliases when value bounds matter.
- Use `TryFrom` for fallible conversions.
- Keep conversion boundaries close to hardware I/O points.

## Change Safety

- Make the smallest possible change that solves the issue.
- Do not silently alter electrical assumptions or pin mappings.
- When modifying hardware-facing behavior, include a brief note in comments only if logic is non-obvious.

## Verification

Before finalizing significant firmware edits, run:

- `cargo check`
- `cargo fmt --check`

## Commits

- Use clear, descriptive commit messages that explain the "what" and "why" of changes.
- For larger changes, consider breaking into multiple commits with focused scopes.
- Use conventional commit message format if possible (e.g., `fix:`, `feat:`, `refactor:`) to indicate the nature of changes.

## Reviewer Instructions

- Verify that changes adhere to the above guidelines.
- Check for unintended side effects, especially around hardware interactions.
- Ensure that any new types or APIs are well-documented and consistent with existing patterns.
