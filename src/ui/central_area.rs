use egui::{Color32, Context, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use rand::Rng;
use std::{
    fmt,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use strum_macros::EnumIter;

use crate::config::language_config::{LanguageKey, translations};

#[derive(EnumIter, Debug)]
/// 中央区有哪些功能
pub enum CentralAreaFunctions {
    /// 主页
    HomePage,

    /// 配置设置
    ConfigSetting,

    /// 关于
    About,
}

impl fmt::Display for CentralAreaFunctions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CentralAreaFunctions::HomePage => write!(f, "{}", translations(&LanguageKey::HomePage)),
            CentralAreaFunctions::ConfigSetting => {
                write!(f, "{}", translations(&LanguageKey::Settings))
            }
            CentralAreaFunctions::About => write!(f, "{}", translations(&LanguageKey::About)),
        }
    }
}

const NUM_CHANNELS: usize = 64;
const NUM_POINTS: usize = 1000;

/// 主功能区需要管理的状态
pub struct CentralAreaState {
    /// 当前显示哪个功能
    pub current_function: CentralAreaFunctions,

    /// 多通道数据测试
    data: Arc<Mutex<Vec<Vec<f64>>>>,
}

impl Default for CentralAreaState {
    fn default() -> Self {
        // 每个通道一个 Vec<f64>
        let mut channels = Vec::with_capacity(NUM_CHANNELS);
        for _ in 0..NUM_CHANNELS {
            channels.push(vec![0.0; NUM_POINTS]);
        }
        let data = Arc::new(Mutex::new(channels));
        // 启动后台线程生成随机数据
        {
            let data_clone = Arc::clone(&data);
            thread::spawn(move || {
                let mut rng = rand::rng();
                let mut idx = 0;
                loop {
                    {
                        let mut locked = data_clone.lock().unwrap();
                        for ch in 0..NUM_CHANNELS {
                            locked[ch][idx] = rng.random::<f64>() + ch as f64;
                        }
                    }
                    idx = (idx + 1) % NUM_POINTS; // 循环写入，实现滚动
                    thread::sleep(Duration::from_millis(5));
                }
            });
        }

        Self {
            current_function: CentralAreaFunctions::HomePage,
            data,
        }
    }
}

pub fn show_central_area(ctx: &Context, ui: &mut Ui, central_area_state: &CentralAreaState) {
    // 告诉 egui 下帧必须重绘
    ctx.request_repaint();
    match central_area_state.current_function {
        CentralAreaFunctions::HomePage => {
            Plot::new("my_plot").view_aspect(1.0).show(ui, |plot_ui| {
                let locked = central_area_state.data.lock().unwrap();
                for (i, channel) in locked.iter().enumerate() {
                    let points: PlotPoints = (0..NUM_POINTS)
                        .map(|j| [j as f64 * 0.001, channel[j]])
                        .collect();
                    let color = Color32::from_rgb(
                        ((i as f32 / NUM_CHANNELS as f32) * 255.0) as u8,
                        100,
                        255 - ((i as f32 / NUM_CHANNELS as f32) * 255.0) as u8,
                    );
                    let line = Line::new(format!("Ch {}", i + 1), points).color(color);
                    plot_ui.line(line);
                }
            });
        }
        CentralAreaFunctions::ConfigSetting => {
            ui.label("设置");
        }
        CentralAreaFunctions::About => {
            ui.label("关于");
        }
    };
}
