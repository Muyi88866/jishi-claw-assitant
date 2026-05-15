use serde::{Deserialize, Serialize};
use tauri::Manager;

/// OpenClaw 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenClawConfig {
    url: String,
    api_key: String,
    model: String,
}

impl Default for OpenClawConfig {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:28789".to_string(),
            api_key: "455a8e1c7e9c1da9070cf1745ae41b63446a6574d2f10310".to_string(),
            model: "openclaw/main".to_string(),
        }
    }
}

/// OpenClaw 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenClaw API 请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

/// OpenClaw API 响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    content: String,
}

/// 连接测试响应
#[derive(Debug, Serialize)]
struct ConnectionStatus {
    connected: bool,
    message: String,
    config: OpenClawConfig,
}

/// Agent 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentInfo {
    id: String,
    name: String,
    model_id: String,
}

/// Models API 响应
#[derive(Debug, Deserialize)]
struct ModelsResponse {
    data: Vec<ModelItem>,
}

#[derive(Debug, Deserialize)]
struct ModelItem {
    id: String,
}

/// 创建 HTTP 客户端
fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .unwrap_or_default()
}

/// 读取配置文件
fn load_config(app: &tauri::AppHandle) -> OpenClawConfig {
    let config_path = app.path().app_config_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("openclaw_config.json");

    if config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str::<OpenClawConfig>(&content) {
                return config;
            }
        }
    }

    OpenClawConfig::default()
}

/// 保存配置文件
fn save_config(app: &tauri::AppHandle, config: &OpenClawConfig) -> Result<(), String> {
    let config_dir = app.path().app_config_dir()
        .map_err(|e| format!("获取配置目录失败: {}", e))?;

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    let config_path = config_dir.join("openclaw_config.json");
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;

    std::fs::write(&config_path, content)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            test_connection,
            send_chat_message,
            get_config,
            save_config_cmd,
            get_system_info,
            get_agents,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("欢迎使用及时claw助手，{}！调用成功 ✅", name)
}

/// 测试与 OpenClaw 的连接
#[tauri::command]
async fn test_connection(app: tauri::AppHandle) -> ConnectionStatus {
    let config = load_config(&app);
    let client = create_client();

    let url = format!("{}/v1/models", config.url.trim_end_matches('/'));

    let mut req_builder = client.get(&url)
        .timeout(std::time::Duration::from_secs(10));

    if !config.api_key.is_empty() {
        req_builder = req_builder.header("Authorization", format!("Bearer {}", config.api_key));
    }

    match req_builder.send().await
    {
        Ok(response) => {
            if response.status().is_success() {
                ConnectionStatus {
                    connected: true,
                    message: "连接成功！OpenClaw 服务正在运行".to_string(),
                    config,
                }
            } else {
                ConnectionStatus {
                    connected: false,
                    message: format!("服务器返回错误状态码: {}", response.status()),
                    config,
                }
            }
        }
        Err(e) => {
            ConnectionStatus {
                connected: false,
                message: format!("连接失败: {}。请确认 OpenClaw 服务已启动，地址: {}", e, config.url),
                config,
            }
        }
    }
}

/// 发送聊天消息到 OpenClaw
#[tauri::command]
async fn send_chat_message(
    app: tauri::AppHandle,
    messages: Vec<ChatMessage>,
    agent_id: Option<String>,
) -> Result<String, String> {
    let config = load_config(&app);
    let client = create_client();

    let url = format!("{}/v1/chat/completions", config.url.trim_end_matches('/'));

    // 如果指定了agent_id，用 openclaw/<agent_id> 格式
    let model = match agent_id {
        Some(ref aid) if !aid.is_empty() => format!("openclaw/{}", aid),
        _ => config.model.clone(),
    };

    let request = ChatRequest {
        model,
        messages,
        stream: false,
    };

    let mut req_builder = client.post(&url)
        .json(&request);

    if !config.api_key.is_empty() {
        req_builder = req_builder.header("Authorization", format!("Bearer {}", config.api_key));
    }

    match req_builder.send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ChatResponse>().await {
                    Ok(chat_response) => {
                        if let Some(choice) = chat_response.choices.first() {
                            Ok(choice.message.content.clone())
                        } else {
                            Err("服务器返回空响应".to_string())
                        }
                    }
                    Err(e) => Err(format!("解析响应失败: {}", e))
                }
            } else {
                let status = response.status();
                let body = response.text().await.unwrap_or_else(|_| "无法读取响应内容".to_string());
                Err(format!("请求失败 ({}): {}", status, body))
            }
        }
        Err(e) => Err(format!("发送请求失败: {}。请确认 OpenClaw 服务已启动", e))
    }
}

/// 获取当前配置
#[tauri::command]
async fn get_config(app: tauri::AppHandle) -> OpenClawConfig {
    load_config(&app)
}

/// 保存配置
#[tauri::command]
async fn save_config_cmd(app: tauri::AppHandle, config: OpenClawConfig) -> Result<(), String> {
    save_config(&app, &config)
}

/// 获取系统信息
#[tauri::command]
async fn get_system_info() -> serde_json::Value {
    serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
    })
}

/// 获取 OpenClaw 可用的 Agent 列表
#[tauri::command]
async fn get_agents(app: tauri::AppHandle) -> Result<Vec<AgentInfo>, String> {
    let config = load_config(&app);
    let client = create_client();

    let url = format!("{}/v1/models", config.url.trim_end_matches('/'));

    let mut req_builder = client.get(&url)
        .timeout(std::time::Duration::from_secs(10));

    if !config.api_key.is_empty() {
        req_builder = req_builder.header("Authorization", format!("Bearer {}", config.api_key));
    }

    match req_builder.send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ModelsResponse>().await {
                    Ok(models) => {
                        let agents: Vec<AgentInfo> = models.data
                            .into_iter()
                            .filter(|m| m.id.starts_with("openclaw/"))
                            .map(|m| {
                                let agent_id = m.id.trim_start_matches("openclaw/").to_string();
                                let name = match agent_id.as_str() {
                                    "main" => "QClaw (默认助手)".to_string(),
                                    "default" => "默认Agent".to_string(),
                                    id => id.to_string(),
                                };
                                AgentInfo {
                                    id: agent_id,
                                    name,
                                    model_id: m.id.clone(),
                                }
                            })
                            .collect();
                        Ok(agents)
                    }
                    Err(e) => Err(format!("解析Agent列表失败: {}", e))
                }
            } else {
                Err(format!("获取Agent列表失败: {}", response.status()))
            }
        }
        Err(e) => Err(format!("连接OpenClaw失败: {}", e))
    }
}
