use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub prompt: String,
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

const SYSTEM_PROMPT: &str = "You are a terminal assistant inside NEXTERM, an AI-powered terminal emulator and DevOps command center. Help users with shell commands, explain errors, and translate natural language to shell commands. Be concise and precise. When translating to commands, output ONLY the command without explanation unless asked.";

#[tauri::command]
pub async fn ai_query(request: AIRequest) -> Result<AIResponse, String> {
    let client = reqwest::Client::new();

    match request.provider.as_str() {
        "ollama" => {
            let ollama_req = OllamaRequest {
                model: request.model.clone(),
                prompt: format!("{}\n\nUser: {}", SYSTEM_PROMPT, request.prompt),
                stream: false,
            };

            let resp = client
                .post("http://localhost:11434/api/generate")
                .json(&ollama_req)
                .send()
                .await
                .map_err(|e| format!("Ollama error: {}", e))?;

            let body: OllamaResponse = resp
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;

            Ok(AIResponse {
                content: body.response,
                provider: "ollama".to_string(),
                model: request.model,
            })
        }
        "openai" => {
            let openai_req = OpenAIRequest {
                model: request.model.clone(),
                messages: vec![
                    OpenAIMessage {
                        role: "system".to_string(),
                        content: SYSTEM_PROMPT.to_string(),
                    },
                    OpenAIMessage {
                        role: "user".to_string(),
                        content: request.prompt,
                    },
                ],
                max_tokens: 1024,
            };

            let resp = client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", request.api_key))
                .json(&openai_req)
                .send()
                .await
                .map_err(|e| format!("OpenAI error: {}", e))?;

            let body: OpenAIResponse = resp
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;

            let content = body
                .choices
                .first()
                .map(|c| c.message.content.clone())
                .unwrap_or_default();

            Ok(AIResponse {
                content,
                provider: "openai".to_string(),
                model: request.model,
            })
        }
        "anthropic" => {
            let body = serde_json::json!({
                "model": request.model,
                "max_tokens": 1024,
                "system": SYSTEM_PROMPT,
                "messages": [{
                    "role": "user",
                    "content": request.prompt
                }]
            });

            let resp = client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &request.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&body)
                .send()
                .await
                .map_err(|e| format!("Anthropic error: {}", e))?;

            let resp_body: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;

            let content = resp_body["content"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string();

            Ok(AIResponse {
                content,
                provider: "anthropic".to_string(),
                model: request.model,
            })
        }
        _ => Err(format!("Unknown provider: {}", request.provider)),
    }
}

#[tauri::command]
pub async fn ai_translate_command(
    prompt: String,
    provider: String,
    model: String,
    api_key: String,
    shell: String,
) -> Result<String, String> {
    let request = AIRequest {
        prompt: format!(
            "Translate this natural language to a {} shell command. Output ONLY the command, nothing else: {}",
            shell, prompt
        ),
        provider,
        model,
        api_key,
        context: None,
    };

    let response = ai_query(request).await?;
    Ok(response.content.trim().to_string())
}

#[tauri::command]
pub async fn ai_explain_command(
    command: String,
    provider: String,
    model: String,
    api_key: String,
) -> Result<String, String> {
    let request = AIRequest {
        prompt: format!("Explain this shell command concisely: {}", command),
        provider,
        model,
        api_key,
        context: None,
    };

    let response = ai_query(request).await?;
    Ok(response.content)
}

#[tauri::command]
pub async fn ai_analyze_error(
    error_output: String,
    command: String,
    provider: String,
    model: String,
    api_key: String,
) -> Result<String, String> {
    let request = AIRequest {
        prompt: format!(
            "The command '{}' produced this error:\n{}\nExplain the error and suggest a fix.",
            command, error_output
        ),
        provider,
        model,
        api_key,
        context: None,
    };

    let response = ai_query(request).await?;
    Ok(response.content)
}
