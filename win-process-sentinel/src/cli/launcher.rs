use std::collections::HashMap;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone)]
struct DevTool {
    app_cmd: Option<&'static str>,
    web_url: Option<&'static str>,
}

#[derive(PartialEq)]
enum LaunchMode {
    AppOnly,
    WebOnly,
    AutoDetect,
}

pub fn run_apps_and_sites(target_string: &str, _detach: bool) {
    let registry = build_dev_registry();

    // Splits on BOTH commas and spaces, trimming any whitespace
    let targets: Vec<&str> = target_string
        .split([',', ' '])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    println!("==========================================================================");
    println!("   🚀 SENTINEL UNIVERSAL DEVELOPER LAUNCHER");
    println!("==========================================================================");

    for raw_target in targets {
        let (clean_target, mode) = parse_target_suffix(raw_target);
        let key = clean_target.to_lowercase();

        if let Some(tool) = registry.get(key.as_str()) {
            match mode {
                LaunchMode::AppOnly => {
                    if let Some(cmd) = tool.app_cmd {
                        println!("🖥️  [FORCED APP] Launching {}: {}", clean_target, cmd);
                        launch_command(cmd);
                    } else {
                        println!("⚠️  [NO APP] {} does not have a desktop app registered. Falling back to web...", clean_target);
                        if let Some(url) = tool.web_url {
                            launch_website(url);
                        }
                    }
                }
                LaunchMode::WebOnly => {
                    if let Some(url) = tool.web_url {
                        println!("🌐 [FORCED WEB] Opening Portal {}: {}", clean_target, url);
                        launch_website(url);
                    } else {
                        println!("⚠️  [NO WEB] {} does not have a web portal registered. Falling back to app...", clean_target);
                        if let Some(cmd) = tool.app_cmd {
                            launch_command(cmd);
                        }
                    }
                }
                LaunchMode::AutoDetect => {
                    if let Some(cmd) = tool.app_cmd {
                        println!("🖥️  [AUTO] Launching App {}: {}", clean_target, cmd);
                        launch_command(cmd);
                    } else if let Some(url) = tool.web_url {
                        println!("🌐 [AUTO] Opening Portal {}: {}", clean_target, url);
                        launch_website(url);
                    }
                }
            }
        } else {
            if mode == LaunchMode::WebOnly || is_raw_url(&clean_target) {
                println!("🌐 [CUSTOM] Opening URL: {}", clean_target);
                launch_website(&clean_target);
            } else {
                println!("🖥️  [CUSTOM] Launching Binary: {}", clean_target);
                launch_command(&format!("start {}", clean_target));
            }
        }
    }

    println!("\n✨ Launch sequence completed!");
}

fn parse_target_suffix(target: &str) -> (String, LaunchMode) {
    let lower = target.to_lowercase();
    if lower.ends_with(".app") {
        (target[..target.len() - 4].to_string(), LaunchMode::AppOnly)
    } else if lower.ends_with(".web") {
        (target[..target.len() - 4].to_string(), LaunchMode::WebOnly)
    } else {
        (target.to_string(), LaunchMode::AutoDetect)
    }
}

fn build_dev_registry() -> HashMap<&'static str, DevTool> {
    let mut m = HashMap::new();

    // AI Models & Tools
    m.insert(
        "chatgpt",
        DevTool {
            app_cmd: None,
            web_url: Some("https://chatgpt.com"),
        },
    );
    m.insert(
        "claude",
        DevTool {
            app_cmd: None,
            web_url: Some("https://claude.ai"),
        },
    );
    m.insert(
        "grok",
        DevTool {
            app_cmd: None,
            web_url: Some("https://x.com/i/grok"),
        },
    );
    m.insert(
        "qwen",
        DevTool {
            app_cmd: None,
            web_url: Some("https://chat.qwenlm.ai"),
        },
    );
    m.insert(
        "gemini",
        DevTool {
            app_cmd: None,
            web_url: Some("https://gemini.google.com"),
        },
    );
    m.insert(
        "deepseek",
        DevTool {
            app_cmd: None,
            web_url: Some("https://chat.deepseek.com"),
        },
    );
    m.insert(
        "ollama",
        DevTool {
            app_cmd: Some("start ollama"),
            web_url: Some("http://localhost:11434"),
        },
    );
    m.insert(
        "v0",
        DevTool {
            app_cmd: None,
            web_url: Some("https://v0.dev"),
        },
    );
    m.insert(
        "bolt",
        DevTool {
            app_cmd: None,
            web_url: Some("https://bolt.new"),
        },
    );
    m.insert(
        "perplexity",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.perplexity.ai"),
        },
    );
    m.insert(
        "huggingface",
        DevTool {
            app_cmd: None,
            web_url: Some("https://huggingface.co"),
        },
    );

    // Databases & Cache
    m.insert(
        "mongodb",
        DevTool {
            app_cmd: Some("start mongodbcompass"),
            web_url: Some("https://cloud.mongodb.com/v2"),
        },
    );
    m.insert(
        "mongo",
        DevTool {
            app_cmd: Some("start mongodbcompass"),
            web_url: Some("https://cloud.mongodb.com/v2"),
        },
    );
    m.insert(
        "redis",
        DevTool {
            app_cmd: Some("start redisinsight"),
            web_url: Some("https://app.redislabs.com"),
        },
    );
    m.insert(
        "supabase",
        DevTool {
            app_cmd: None,
            web_url: Some("https://supabase.com/dashboard/projects"),
        },
    );
    m.insert(
        "postgres",
        DevTool {
            app_cmd: Some("start pgadmin4"),
            web_url: Some("https://neon.tech"),
        },
    );
    m.insert(
        "neon",
        DevTool {
            app_cmd: None,
            web_url: Some("https://console.neon.tech"),
        },
    );

    // API & QA Testing
    m.insert(
        "postman",
        DevTool {
            app_cmd: Some("start postman:"),
            web_url: Some("https://web.postman.co"),
        },
    );
    m.insert(
        "insomnia",
        DevTool {
            app_cmd: Some("start insomnia:"),
            web_url: None,
        },
    );
    m.insert(
        "jest",
        DevTool {
            app_cmd: None,
            web_url: Some("https://jestjs.io/docs/getting-started"),
        },
    );
    m.insert(
        "puppeteer",
        DevTool {
            app_cmd: None,
            web_url: Some("https://pptr.dev"),
        },
    );
    m.insert(
        "playwright",
        DevTool {
            app_cmd: None,
            web_url: Some("https://playwright.dev/docs/intro"),
        },
    ); // =========================================================================
       // 🤖 AI ORCHESTRATION & HARDWARE
       // =========================================================================
    m.insert(
        "langchain",
        DevTool {
            app_cmd: None,
            web_url: Some("https://js.langchain.com"),
        },
    );
    m.insert(
        "langgraph",
        DevTool {
            app_cmd: None,
            web_url: Some("https://langchain-ai.github.io/langgraph"),
        },
    );
    m.insert(
        "cuda",
        DevTool {
            app_cmd: None,
            web_url: Some("https://developer.nvidia.com/cuda-toolkit"),
        },
    );
    m.insert(
        "nvidia",
        DevTool {
            app_cmd: None,
            web_url: Some("https://developer.nvidia.com"),
        },
    );
    m.insert(
        "amd",
        DevTool {
            app_cmd: None,
            web_url: Some("https://rocm.docs.amd.com"),
        },
    ); // AMD ROCm developer docs

    // =========================================================================
    // ⚙️ LANGUAGES & BACKEND FRAMEWORKS
    // =========================================================================
    m.insert(
        "rust",
        DevTool {
            app_cmd: None,
            web_url: Some("https://doc.rust-lang.org/book/"),
        },
    );
    m.insert(
        "ts",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.typescriptlang.org"),
        },
    );
    m.insert(
        "typescript",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.typescriptlang.org"),
        },
    );
    m.insert(
        "cpp",
        DevTool {
            app_cmd: None,
            web_url: Some("https://en.cppreference.com/w/"),
        },
    );
    m.insert(
        "c++",
        DevTool {
            app_cmd: None,
            web_url: Some("https://en.cppreference.com/w/"),
        },
    );
    m.insert(
        "python",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.python.org/3/"),
        },
    );
    m.insert(
        "java",
        DevTool {
            app_cmd: None,
            web_url: Some("https://dev.java"),
        },
    );
    m.insert(
        "django",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.djangoproject.com"),
        },
    );
    m.insert(
        "fastapi",
        DevTool {
            app_cmd: None,
            web_url: Some("https://fastapi.tiangolo.com"),
        },
    );
    m.insert(
        "actix",
        DevTool {
            app_cmd: None,
            web_url: Some("https://actix.rs"),
        },
    );

    // =========================================================================
    // 🖥️ DESKTOP APP FRAMEWORKS (GUI)
    // =========================================================================
    m.insert(
        "tauri",
        DevTool {
            app_cmd: None,
            web_url: Some("https://tauri.app"),
        },
    );
    m.insert(
        "electron",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.electronjs.org"),
        },
    );

    // =========================================================================
    // 🎮 GAME ENGINES
    // =========================================================================
    m.insert(
        "unity",
        DevTool {
            app_cmd: Some("start unityhub"),
            web_url: Some("https://unity.com"),
        },
    );
    m.insert(
        "unreal",
        DevTool {
            app_cmd: Some("start unreal"),
            web_url: Some("https://www.unrealengine.com"),
        },
    );
    m.insert(
        "godot",
        DevTool {
            app_cmd: Some("start godot"),
            web_url: Some("https://godotengine.org"),
        },
    );

    // =========================================================================
    // ☁️ CLOUD, DEVOPS & INFRASTRUCTURE
    // =========================================================================
    m.insert(
        "aws",
        DevTool {
            app_cmd: None,
            web_url: Some("https://us-east-1.console.aws.amazon.com/console/home"),
        },
    );
    m.insert(
        "k8s",
        DevTool {
            app_cmd: None,
            web_url: Some("https://kubernetes.io/docs/home/"),
        },
    );
    m.insert(
        "kubernetes",
        DevTool {
            app_cmd: None,
            web_url: Some("https://kubernetes.io/docs/home/"),
        },
    );
    m.insert(
        "nginx",
        DevTool {
            app_cmd: None,
            web_url: Some("https://nginx.org/en/docs/"),
        },
    );
    m.insert(
        "pm2",
        DevTool {
            app_cmd: None,
            web_url: Some("https://pm2.keymetrics.io"),
        },
    );
    m.insert(
        "turborepo",
        DevTool {
            app_cmd: None,
            web_url: Some("https://turbo.build/repo"),
        },
    );
    m.insert(
        "turbo",
        DevTool {
            app_cmd: None,
            web_url: Some("https://turbo.build/repo"),
        },
    );

    // =========================================================================
    // 📬 MESSAGE QUEUES & EVENT BROKERS
    // =========================================================================
    m.insert(
        "rabbitmq",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.rabbitmq.com"),
        },
    );
    m.insert(
        "bullmq",
        DevTool {
            app_cmd: None,
            web_url: Some("https://bullmq.io"),
        },
    );

    // =========================================================================
    // 🔄 AUTOMATION & LOAD TESTING
    // =========================================================================
    m.insert(
        "n8n",
        DevTool {
            app_cmd: None,
            web_url: Some("http://localhost:5678"),
        },
    ); // Defaults to local instance
    m.insert(
        "n8n.web",
        DevTool {
            app_cmd: None,
            web_url: Some("https://n8n.io"),
        },
    ); // Forces the public site
    m.insert(
        "artillery",
        DevTool {
            app_cmd: None,
            web_url: Some("https://artillery.io"),
        },
    );

    // Containers & Infrastructure
    m.insert(
        "docker",
        DevTool {
            app_cmd: Some("start docker:"),
            web_url: Some("https://hub.docker.com"),
        },
    );
    m.insert(
        "pm2",
        DevTool {
            app_cmd: None,
            web_url: Some("https://pm2.io/docs/runtime/overview/"),
        },
    );
    m.insert(
        "vercel",
        DevTool {
            app_cmd: None,
            web_url: Some("https://vercel.com/dashboard"),
        },
    );
    // Make 'github' default to the website
    m.insert(
        "github",
        DevTool {
            app_cmd: None,
            web_url: Some("https://github.com"),
        },
    );

    // Add a specific alias for the desktop app
    m.insert(
        "github.app",
        DevTool {
            app_cmd: Some("start github:"),
            web_url: None,
        },
    );
    m.insert(
        "githubdesk",
        DevTool {
            app_cmd: Some("start github:"),
            web_url: None,
        },
    );
    m.insert(
        "npm",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.npmjs.com"),
        },
    );

    // Editors & Terminals
    m.insert(
        "code",
        DevTool {
            app_cmd: Some("code"),
            web_url: Some("https://vscode.dev"),
        },
    );
    m.insert(
        "cursor",
        DevTool {
            app_cmd: Some("cursor"),
            web_url: None,
        },
    );
    m.insert(
        "zed",
        DevTool {
            app_cmd: Some("zed"),
            web_url: None,
        },
    );
    m.insert(
        "terminal",
        DevTool {
            app_cmd: Some("wt"),
            web_url: None,
        },
    );

    // Design & Workspace Communication
    m.insert(
        "figma",
        DevTool {
            app_cmd: Some("start figma:"),
            web_url: Some("https://www.figma.com"),
        },
    );
    m.insert(
        "discord",
        DevTool {
            app_cmd: Some("start discord:"),
            web_url: Some("https://discord.com/app"),
        },
    );
    m.insert(
        "slack",
        DevTool {
            app_cmd: Some("start slack:"),
            web_url: Some("https://app.slack.com"),
        },
    );
    m.insert(
        "notion",
        DevTool {
            app_cmd: Some("start notion:"),
            web_url: Some("https://www.notion.so"),
        },
    );
    m.insert(
        "spotify",
        DevTool {
            app_cmd: Some("start spotify:"),
            web_url: Some("https://open.spotify.com"),
        },
    ); // =========================================================================
       // ⚛️ FRONTEND FRAMEWORKS & UI LIBRARIES
       // =========================================================================
    m.insert(
        "react",
        DevTool {
            app_cmd: None,
            web_url: Some("https://react.dev"),
        },
    );
    m.insert(
        "nextjs",
        DevTool {
            app_cmd: None,
            web_url: Some("https://nextjs.org/docs"),
        },
    );
    m.insert(
        "next",
        DevTool {
            app_cmd: None,
            web_url: Some("https://nextjs.org/docs"),
        },
    );
    m.insert(
        "vue",
        DevTool {
            app_cmd: None,
            web_url: Some("https://vuejs.org/guide/introduction.html"),
        },
    );
    m.insert(
        "nuxt",
        DevTool {
            app_cmd: None,
            web_url: Some("https://nuxt.com/docs"),
        },
    );
    m.insert(
        "svelte",
        DevTool {
            app_cmd: None,
            web_url: Some("https://svelte.dev/docs"),
        },
    );
    m.insert(
        "tailwind",
        DevTool {
            app_cmd: None,
            web_url: Some("https://tailwindcss.com/docs"),
        },
    );
    m.insert(
        "shadcn",
        DevTool {
            app_cmd: None,
            web_url: Some("https://ui.shadcn.com"),
        },
    );
    m.insert(
        "storybook",
        DevTool {
            app_cmd: None,
            web_url: Some("https://storybook.js.org"),
        },
    );
    m.insert(
        "radix",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.radix-ui.com"),
        },
    );

    // =========================================================================
    // 📱 MOBILE & CROSS-PLATFORM
    // =========================================================================
    m.insert(
        "reactnative",
        DevTool {
            app_cmd: None,
            web_url: Some("https://reactnative.dev"),
        },
    );
    m.insert(
        "expo",
        DevTool {
            app_cmd: None,
            web_url: Some("https://expo.dev"),
        },
    );
    m.insert(
        "flutter",
        DevTool {
            app_cmd: None,
            web_url: Some("https://flutter.dev"),
        },
    );
    m.insert(
        "swift",
        DevTool {
            app_cmd: None,
            web_url: Some("https://developer.apple.com/swift/"),
        },
    );
    m.insert(
        "kotlin",
        DevTool {
            app_cmd: None,
            web_url: Some("https://kotlinlang.org/docs/home.html"),
        },
    );

    // =========================================================================
    // 🎥 MEDIA STREAMING, VIDEO & GRAPHICS
    // =========================================================================
    m.insert(
        "webrtc",
        DevTool {
            app_cmd: None,
            web_url: Some("https://webrtc.org"),
        },
    );
    m.insert(
        "ffmpeg",
        DevTool {
            app_cmd: None,
            web_url: Some("https://ffmpeg.org/documentation.html"),
        },
    );
    m.insert(
        "obs",
        DevTool {
            app_cmd: Some("start obs64"),
            web_url: Some("https://obsproject.com"),
        },
    );
    m.insert(
        "threejs",
        DevTool {
            app_cmd: None,
            web_url: Some("https://threejs.org/docs/"),
        },
    );
    m.insert(
        "webgl",
        DevTool {
            app_cmd: None,
            web_url: Some("https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API"),
        },
    );
    m.insert(
        "phaser",
        DevTool {
            app_cmd: None,
            web_url: Some("https://phaser.io"),
        },
    );
    m.insert(
        "canvas",
        DevTool {
            app_cmd: None,
            web_url: Some("https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API"),
        },
    );

    // =========================================================================
    // 🔗 API DOCS & GRAPHQL
    // =========================================================================
    m.insert(
        "graphql",
        DevTool {
            app_cmd: None,
            web_url: Some("https://graphql.org"),
        },
    );
    m.insert(
        "apollo",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.apollographql.com/docs/"),
        },
    );
    m.insert(
        "swagger",
        DevTool {
            app_cmd: None,
            web_url: Some("https://swagger.io"),
        },
    );
    m.insert(
        "openapi",
        DevTool {
            app_cmd: None,
            web_url: Some("https://swagger.io/specification/"),
        },
    );

    // =========================================================================
    // 🪙 WEB3, CRYPTO & TOKENS
    // =========================================================================
    m.insert(
        "solidity",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.soliditylang.org"),
        },
    );
    m.insert(
        "hardhat",
        DevTool {
            app_cmd: None,
            web_url: Some("https://hardhat.org"),
        },
    );
    m.insert(
        "ethers",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.ethers.org/v6/"),
        },
    );
    m.insert(
        "metamask",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.metamask.io"),
        },
    );
    m.insert(
        "web3",
        DevTool {
            app_cmd: None,
            web_url: Some("https://web3js.readthedocs.io/"),
        },
    );

    // =========================================================================
    // 🚀 CI/CD & AUTOMATION
    // =========================================================================
    m.insert(
        "jenkins",
        DevTool {
            app_cmd: None,
            web_url: Some("https://www.jenkins.io/doc/"),
        },
    );
    m.insert(
        "actions",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.github.com/en/actions"),
        },
    );
    m.insert(
        "gitlabci",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.gitlab.com/ee/ci/"),
        },
    );
    m.insert(
        "terraform",
        DevTool {
            app_cmd: None,
            web_url: Some("https://developer.hashicorp.com/terraform/docs"),
        },
    );
    m.insert(
        "ansible",
        DevTool {
            app_cmd: None,
            web_url: Some("https://docs.ansible.com"),
        },
    );

    // =========================================================================
    // 🔥 BACKEND-AS-A-SERVICE (BaaS)
    // =========================================================================
    m.insert(
        "firebase",
        DevTool {
            app_cmd: None,
            web_url: Some("https://firebase.google.com/docs"),
        },
    );
    m.insert(
        "appwrite",
        DevTool {
            app_cmd: None,
            web_url: Some("https://appwrite.io/docs"),
        },
    );
    m.insert(
        "heroku",
        DevTool {
            app_cmd: None,
            web_url: Some("https://dashboard.heroku.com"),
        },
    );
    m.insert(
        "digitalocean",
        DevTool {
            app_cmd: None,
            web_url: Some("https://cloud.digitalocean.com"),
        },
    );

    // =========================================================================
    // 📦 BACKEND CORE & RUNTIMES
    // =========================================================================
    m.insert(
        "nodejs",
        DevTool {
            app_cmd: None,
            web_url: Some("https://nodejs.org/en/docs/"),
        },
    );
    m.insert(
        "node",
        DevTool {
            app_cmd: None,
            web_url: Some("https://nodejs.org/en/docs/"),
        },
    );
    m.insert(
        "bun",
        DevTool {
            app_cmd: None,
            web_url: Some("https://bun.sh/docs"),
        },
    );
    m.insert(
        "deno",
        DevTool {
            app_cmd: None,
            web_url: Some("https://deno.land/manual"),
        },
    );
    m.insert(
        "express",
        DevTool {
            app_cmd: None,
            web_url: Some("https://expressjs.com"),
        },
    );

    m
}

fn is_raw_url(target: &str) -> bool {
    let t = target.to_lowercase();
    t.starts_with("http://")
        || t.starts_with("https://")
        || t.contains("localhost")
        || t.starts_with("127.0.0.1")
        || t.contains(".com")
        || t.contains(".org")
        || t.contains(".net")
        || t.contains(".io")
        || t.contains(".dev")
        || t.contains(".app")
        || t.contains(".tech")
        || t.contains(".sh")
}

fn launch_website(target: &str) {
    let url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("https://{}", target)
    };

    let _ = Command::new("cmd")
        .args(["/C", "start", "", &url])
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn launch_command(cmd: &str) {
    let _ = Command::new("cmd")
        .args(["/C", cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}
