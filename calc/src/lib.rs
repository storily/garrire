use libc::c_char;
use rhai::{Engine, RegisterFn};
use std::{ffi::{CStr, CString}};

#[no_mangle]
pub unsafe extern "C" fn eval(expr: *const c_char) -> *mut c_char {
    fn out(s: &str) -> *mut c_char {
        CString::new(s).unwrap().into_raw()
    }

    let c_expr = CStr::from_ptr(expr);
    let r_expr = match c_expr.to_str() {
        Ok(s) => s,
        Err(_) => return out("err:expr is not a valid string"),
    };

    out(&match calc(r_expr) {
        Ok(s) => s,
        Err(e) => format!("err:{}", e),
    })
}

#[no_mangle]
pub unsafe extern "C" fn free_output(out: *mut c_char) {
    CString::from_raw(out);
}

pub fn calc(expr: &str) -> Result<String, String> {
    let mut engine = Engine::new();
    engine.register_fn("+", |a: i64, b: f64| -> f64 { (a as f64) + b });
    engine.register_fn("+", |a: f64, b: i64| -> f64 { a + (b as f64) });
    engine.register_fn("-", |a: i64, b: f64| -> f64 { (a as f64) - b });
    engine.register_fn("-", |a: f64, b: i64| -> f64 { a - (b as f64) });
    engine.register_fn("*", |a: i64, b: f64| -> f64 { (a as f64) * b });
    engine.register_fn("*", |a: f64, b: i64| -> f64 { a * (b as f64) });
    engine.register_fn("/", |a: i64, b: f64| -> f64 { (a as f64) / b });
    engine.register_fn("/", |a: f64, b: i64| -> f64 { a / (b as f64) });

    engine
        .eval::<f64>(expr)
        .map(|v| v.to_string())
        .or_else(|_| engine.eval::<i64>(expr).map(|v| v.to_string()))
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod test {
    use super::eval;

    fn ok(s: &'static str) -> Result<String, String> {
        Ok(String::from(s))
    }

    fn err(s: &'static str) -> Result<String, String> {
        Err(String::from(s))
    }

    #[test]
    fn addition() {
        assert_eq!(eval("1 + 5"), ok("6"));
        assert_eq!(eval("1.23 + 5.67"), ok("6.9"));
        assert_eq!(
            eval("9 + 4 + 9 + 2 + 4 + 7 + 4 + 9 + 12 + 10 + 11 + 2"),
            ok("83")
        );
    }

    #[test]
    fn subtraction() {
        assert_eq!(eval("1 - 5"), ok("-4"));
        assert_eq!(eval("72.53 - 19.28"), ok("53.25"));
    }

    // TODO: make pretty instead
    #[test]
    fn ugly_float() {
        assert_eq!(eval("72.53 - 19.27"), ok("53.260000000000005"));
    }

    #[test]
    fn multiplication() {
        assert_eq!(eval("19 * 40"), ok("760"));
        assert_eq!(eval("51.12 * 25.1"), ok("1283.112"));
    }

    #[test]
    fn division() {
        assert_eq!(eval("1 / 5"), ok("0"));
        assert_eq!(eval("1.0 / 5"), ok("0.2"));
        assert_eq!(eval("88.67 / 82.1"), ok("1.0800243605359319"));
    }

    #[test]
    fn error() {
        assert_eq!(
            eval("not_found"),
            err("Variable not found: 'not_found' (line 1, position 1)")
        );
    }
}
