def test_top_level():
    import pyo3_sub
    assert pyo3_sub.__doc__ == "Top-level module for diffsol bindings by type"

def test_f32():
    from pyo3_sub import module_f32 as ps
    assert ps.__doc__ == "diffsol wrapper for f32 type"

def test_f64():
    from pyo3_sub import module_f64 as ps
    assert ps.__doc__ == "diffsol wrapper for f64 type"

def test_classes():
    from pyo3_sub import module_f64 as ps
    assert ps.Builder.__doc__.startswith("Builder does Y")
    assert ps.Problem.__doc__.startswith("Problem does X")
    assert ps.Builder.get_problem.__doc__.startswith("Get the problem")
