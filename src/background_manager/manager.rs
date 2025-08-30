use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use singlyton::SingletonUninit;
use tokio::{
    fs::File,
    io::AsyncReadExt,
    sync::mpsc::{Receiver, Sender, error::TryRecvError},
};
use tracing::{debug, info};

/// 后台线程的sender和receiver
pub static SEND_TO_UI: SingletonUninit<Sender<i64>> = SingletonUninit::uninit();
pub static RECEIVE_UI_MESSAGE: SingletonUninit<Receiver<i64>> = SingletonUninit::uninit();

/// 接收Ui发送的消息并作相应处理
pub async fn background_task_dispatcher() {
    loop {
        match RECEIVE_UI_MESSAGE.get_mut().try_recv() {
            Ok(command) => {
                //根据Ui的消息作相应处理,实际应该用枚举进行模式匹配
                debug!("Ui command:{command}");
                if command == 0 {
                    // 模拟耗时计算
                    info!("开始耗时计算");
                    let result = simulate_expensive_computation().await;
                    info!("耗时计算结束");
                    let _ = SEND_TO_UI.get().send(result).await;
                } else if command == 1 {
                    // 模拟异步I/O
                    let f = File::open("foo.txt").await;
                    match f {
                        Ok(mut f) => {
                            let mut buffer = Vec::new();
                            let n = f.read_to_end(&mut buffer).await;
                            match n {
                                Ok(n) => {
                                    // 返回读取到的字节数
                                    let _ = SEND_TO_UI.get().send(n as i64).await;
                                }
                                Err(_) => {
                                    // 错误就直接返回-1
                                    let _ = SEND_TO_UI.get().send(-1).await;
                                }
                            }
                        }
                        Err(_) => {
                            // 错误就直接返回-1
                            let _ = SEND_TO_UI.get().send(-1).await;
                        }
                    }
                } else {
                    // 直接返回Ui发过来的命令
                    let _ = SEND_TO_UI.get().send(command).await;
                }
            }
            Err(e) => match e {
                TryRecvError::Empty => {
                    //这个错误不需要处理
                }
                TryRecvError::Disconnected => panic!("未创建和Ui线程的mpsc"),
            },
        }
    }
}

/// 使用rayon进行计算耗时的任务
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = simulate_expensive_computation();
/// ```
async fn simulate_expensive_computation() -> i64 {
    let (send, recv) = tokio::sync::oneshot::channel();
    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Compute the sum on multiple threads.
        let sum: i64 = (0..1000000).collect::<Vec<i64>>().par_iter().sum();
        // Send the result back to Tokio.
        let _ = send.send(sum);
    });
    // Wait for the rayon task.
    recv.await.expect("Panic in rayon::spawn")
}
