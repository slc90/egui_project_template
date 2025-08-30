use chrono::Local;

/// 获取当前时间，转化为字符串，格式为 年-月-日-时-分-秒
pub fn get_current_time_format_string() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d-%H-%M-%S").to_string()
}

/// 只是用于测试的例子
///
/// # Arguments
///
/// - `left` (`u64`) - Describe this parameter.
/// - `right` (`u64`) - Describe this parameter.
///
/// # Returns
///
/// - `u64` - Describe the return value.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = add();
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_success() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_fail() {
        let result = add(2, 3);
        assert_ne!(result, 4);
    }

    #[test]
    fn pretty_assertions_example() {
        let result = add(2, 3);
        assert_eq!(result, 4);
    }
}
