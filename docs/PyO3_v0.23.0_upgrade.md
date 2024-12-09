# PyO3 v0.23.0 Upgrade

## Phase out the usage of GIL-refs

According to the PyO3 0.21.0 relase note,

> This release introduces a substantial new direction for PyO3's API. The `Bound<'py, T>` smart pointer type
> has been added that replaces "GIL Refs" such as `&'py PyAny` and `&'py PyList` with smart-pointer forms
> `Bound<'py, PyAny>` and `Bound<'py, PyList>`. This new smart pointer brings ownership out of PyO3's internals
> and into user control. This has been done for sake of both performance and soundness.

Thus, the usage of `.as_ref(py)` needs to be phased out and replaced by `.bind(py)`:

```diff
     pub fn __repr__(&self, py: Python) -> PyResult<String> {
-        let set_name = self.set_name.as_ref(py).to_str()?;
-        let slot = self.slot.as_ref(py).to_str()?;
-        let main_stat = self.main_stat.0.as_ref(py).to_str()?;
+        let set_name = self.set_name.bind(py).to_str()?;
+        let slot = self.slot.bind(py).to_str()?;
+        let main_stat = self.main_stat.0.bind(py).to_str()?;
         let main_stat_value = self.main_stat.1;
```

## Use `Bound<T>` in method arguments and return type

Explicitly use `Bound<'py, T>` in the return type of `__dict__`:

```diff
     #[getter]
-    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
+    pub fn __dict__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
         let dict = PyDict::new(py);
-        let name_str = self.name.as_ref(py).to_str()?;
+        let name_str = self.name.bind(py).to_str()?;
         dict.set_item("name", name_str)?;
         if let Some(config) = &self.config {
-            dict.set_item("config", config.as_ref(py))?;
+            dict.set_item("config", config.bind(py))?;
         } else {
             dict.set_item("config", py.None())?;
         }
-        Ok(dict.into())
+        Ok(dict)
     }
```

Also apply `Bound<T>` to method argument:

```diff
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -25,7 +25,7 @@ use crate::applications::output::transformative_damage::PyTransformativeDamage;
 import_exception!(json, JSONDecodeError);

 #[pymodule]
-fn _python_genshin_artifact(py: Python<'_>, m: &PyModule) -> PyResult<()> {
+fn _python_genshin_artifact(py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
     m.add("JSONDecodeError", py.get_type::<JSONDecodeError>())?;
     m.add_function(wrap_pyfunction!(get_damage_analysis, m)?)?;
     m.add_function(wrap_pyfunction!(get_transformative_damage, m)?)?;
```

## References

- [PyO3 Migration Guide](https://pyo3.rs/v0.23.0/migration.html#from-020-to-021)
- [pydantic-core#1222 - Upgrade to PyO3 0.21 beta](https://github.com/pydantic/pydantic-core/pull/1222/files#diff-2e9d962a08321605940b5a657135052fbcef87b5e360662bb527c96d9a615542)
- [pydantic-core#1556 - Upgrade to PyO3 0.23 (minimal)](https://github.com/pydantic/pydantic-core/pull/1556/files)
- [pydantic-core#1450 - Upgrade to PyO3 0.23 head (WIP)](https://github.com/pydantic/pydantic-core/pull/1450/files)
