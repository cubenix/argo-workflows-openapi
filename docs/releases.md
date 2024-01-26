# Releases

You can find the most recent version under [Github release](https://github.com/gauravgahlot/argo-workflows-openapi/releases).

## Versioning

Versions are expressed as `x.y.z`, where `x` is the major version, `y` is the minor version, and `z` is the patch version,
following Semantic Versioning terminology.

> ⚠️ Note: Argo Workflows does not use Semantic Versioning. Minor versions may contain breaking changes. Patch versions only 
contain bug fixes and minor features.

## Argo Workflows - Kubernetes Compatibility Matrix

| Argo Workflows \ Kubernetes | 1.17 | 1.18 | 1.19 | 1.20 | 1.21 | 1.22 | 1.23 | 1.24 | 1.25 | 1.26 | 1.27 |
|-----------------------|------|------|------|------|------|------|------|------|------|------|------|
| **3.5**           | `x` | `x` | `x` | `?` | `?` | `?` | `?` | `?` | `✓` | `✓` | `✓` |
| **3.4**           | `x` | `x` | `x` | `?` | `✓` | `✓` | `✓` | `✓` | `✓` | `✓` | `✓` |
| **3.3**           | `?` | `?` | `?` | `?` | `✓` | `✓` | `✓` | `?` | `?` | `?` | `?` |
| **3.2**           | `?` | `?` | `✓` | `✓` | `✓` | `?` | `?` | `?` | `?` | `?` | `?` |
| **3.1**           | `✓` | `✓` | `✓` | `?` | `?` | `?` | `?` | `?` | `?` | `?` | `?` |

* `✓` Fully supported versions.
* `?` Due to breaking changes might not work. Also, we haven't thoroughly tested against this version.
* `✕` Unsupported versions.

## Argo Workflows OpenAPI - Argo Workflows Compatibility Matrix

| OpenAPI \ Argo Workflows | 3.5 |
|--------------------------|-----|
| **0.1.0**                | `✓` |
