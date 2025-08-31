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
    // rstest相关见 https://github.com/la10736/rstest
    use rstest::*;
    use std::{net::SocketAddr, time::Duration};

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

    #[fixture]
    fn fixture() -> u32 {
        42
    }

    #[rstest]
    fn should_success(fixture: u32) {
        assert_eq!(fixture, 42);
    }

    #[rstest]
    fn should_fail(fixture: u32) {
        assert_ne!(fixture, 42);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 1, 2)]
    #[case(2, 1, 3)]
    #[case(3, 2, 5)]
    #[case(4, 3, 7)]
    fn test_add_multiple_cases(#[case] input1: u64, #[case] input2: u64, #[case] expected: u64) {
        assert_eq!(expected, add(input1, input2))
    }

    #[derive(Debug, PartialEq)]
    enum State {
        Init,
        Start,
        Processing,
        Terminated,
    }

    #[derive(Debug, PartialEq)]
    enum Event {
        Error,
        Fatal,
    }

    impl State {
        pub fn process(self, event: Event) -> State {
            match event {
                Event::Error => State::Terminated,
                Event::Fatal => Self::Terminated,
            }
        }
    }

    #[rstest]
    fn combination_examples(
        #[values(State::Init, State::Start, State::Processing)] state: State,
        #[values(Event::Error, Event::Fatal)] event: Event,
    ) {
        assert_eq!(State::Terminated, state.process(event))
    }

    #[rstest]
    #[case("1.2.3.4:8080", 8080)]
    #[case("127.0.0.1:9000", 9000)]
    fn auto_conversion_example(#[case] addr: SocketAddr, #[case] expected: u16) {
        assert_eq!(expected, addr.port());
    }

    #[rstest]
    #[tokio::test]
    #[case(async { 18 }, async { 6 }, 3)]
    async fn my_async_test(
        #[future]
        #[case]
        base: u32,
        #[future]
        #[case]
        div: u32,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, base.await / div.await);
    }

    async fn delayed_sum(a: u32, b: u32, delay: Duration) -> u32 {
        tokio::time::sleep(delay).await;
        a + b
    }

    #[rstest]
    #[tokio::test]
    #[timeout(Duration::from_millis(80))]
    async fn timeout_example() {
        assert_eq!(4, delayed_sum(2, 2, Duration::from_millis(100)).await);
    }
}
