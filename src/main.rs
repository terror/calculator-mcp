use {
  rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::{Parameters}},
    model::{ServerCapabilities, ServerInfo},
    schemars::{self, JsonSchema},
    tool, tool_handler, tool_router,
    transport::io::stdio,
  },
  serde::Deserialize,
  std::process,
};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SumRequest {
  #[schemars(description = "the left hand side number")]
  pub a: i32,
  #[schemars(description = "the right hand side number")]
  pub b: i32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SubRequest {
  #[schemars(description = "the left hand side number")]
  pub a: i32,
  #[schemars(description = "the right hand side number")]
  pub b: i32,
}

#[derive(Debug, Clone)]
pub struct Calculator {
  tool_router: ToolRouter<Self>,
}

impl Default for Calculator {
  fn default() -> Self {
    Self::new()
  }
}

#[tool_handler]
impl ServerHandler for Calculator {
  fn get_info(&self) -> ServerInfo {
    ServerInfo {
      capabilities: ServerCapabilities::builder().enable_tools().build(),
      instructions: Some("A simple calculator".into()),
      ..Default::default()
    }
  }
}

#[tool_router]
impl Calculator {
  pub fn new() -> Self {
    Self {
      tool_router: Self::tool_router(),
    }
  }

  #[tool(description = "Calculate the sum of two numbers")]
  fn sum(
    &self,
    Parameters(SumRequest { a, b }): Parameters<SumRequest>,
  ) -> String {
      (a + b).to_string()
  }

  #[tool(description = "Calculate the difference of two numbers")]
  fn sub(
    &self,
    Parameters(SubRequest { a, b }): Parameters<SubRequest>,
  ) -> String {
      (a - b).to_string()
  }
}

async fn run() -> Result {
  let server = Calculator::new();
  let service = server.serve(stdio()).await?;
  service.waiting().await?;
  Ok(())
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() {
  if let Err(error) = run().await {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
