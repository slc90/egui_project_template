use singlyton::SingletonUninit;
use tokio::sync::mpsc::{Receiver, Sender, error::TryRecvError};

/// 后台线程的sender和receiver
pub static SEND_TO_UI: SingletonUninit<Sender<i64>> = SingletonUninit::uninit();
pub static RECEIVE_UI_MESSAGE: SingletonUninit<Receiver<i64>> = SingletonUninit::uninit();

/// 接收Ui发送的消息并作相应处理
pub async fn background_task_dispatcher() {
    loop {
        match RECEIVE_UI_MESSAGE.get_mut().try_recv() {
            Ok(command) => {
                println!("Ui command:{command}");
                //根据Ui的消息作相应处理,实际应该用枚举进行模式匹配
                let _ = SEND_TO_UI.get().send(command).await;
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
