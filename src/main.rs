mod server;

use std::sync::Arc;

use clap::{Parser, Subcommand};
use longport::{Config, QuoteContext, TradeContext};
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_mcpserver::{McpServer, sse::sse_endpoint, stdio::stdio};
use server::Longport;

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the server with stdio.
    Stdio,
    /// Run the server with SSE.
    Sse {
        /// Bind address for the server.
        #[clap(default_value = "127.0.0.1:8000")]
        bind: String,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Use verbose output
    #[clap(short, long, default_value = "false")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    if cli.verbose {
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .init();
    }

    let config = Arc::new(Config::from_env()?.dont_print_quote_packages());
    let (quote_context, _) = QuoteContext::try_new(config.clone()).await?;
    let (trade_context, _) = TradeContext::try_new(config.clone()).await?;

    match cli.command {
        Commands::Stdio => {
            let server = McpServer::new().tools(Longport::new(quote_context, trade_context));
            stdio(server).await?;
        }
        Commands::Sse { bind } => {
            let listener = TcpListener::bind(&bind);
            let app = Route::new()
                .at(
                    "/sse",
                    sse_endpoint(move || {
                        let tools = Longport::new(quote_context.clone(), trade_context.clone());
                        McpServer::new().tools(tools)
                    }),
                )
                .with(Cors::new());
            Server::new(listener).run(app).await?;
        }
    }

    Ok(())
}
