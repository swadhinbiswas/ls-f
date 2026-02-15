use std::collections::HashMap;

/// An icon entry: the Nerd Font glyph and its ANSI color code.
#[derive(Debug, Clone)]
pub struct IconEntry {
    pub icon: &'static str,
    pub color: &'static str,
}

impl IconEntry {
    const fn new(icon: &'static str, color: &'static str) -> Self {
        Self { icon, color }
    }

    /// Return the icon wrapped in ANSI color escape codes.
    pub fn colored(&self) -> String {
        format!("\x1b[{}m{}\x1b[0m", self.color, self.icon)
    }

    /// Return the icon without color.
    pub fn plain(&self) -> String {
        self.icon.to_string()
    }
}

// Default icon for unknown files
pub const DEFAULT_ICON: IconEntry = IconEntry::new("\u{f016}", "37"); //

// Directory icons
pub const DIR_ICON: IconEntry = IconEntry::new("\u{f115}", "1;34"); //
pub const DIR_HIDDEN_ICON: IconEntry = IconEntry::new("\u{f114}", "34"); //

// Symlink icon
pub const SYMLINK_ICON: IconEntry = IconEntry::new("\u{f0c1}", "36"); //

// Executable icon
pub const EXEC_ICON: IconEntry = IconEntry::new("\u{f489}", "1;32"); //

// Pipe / socket / device icons
pub const PIPE_ICON: IconEntry = IconEntry::new("\u{f731}", "33"); //
pub const SOCKET_ICON: IconEntry = IconEntry::new("\u{f6a7}", "35"); //
pub const BLOCK_DEV_ICON: IconEntry = IconEntry::new("\u{f0a0}", "1;33"); //
pub const CHAR_DEV_ICON: IconEntry = IconEntry::new("\u{e601}", "1;33"); //

/// Build the complete extension -> IconEntry map.
/// Uses a function so the map is built once and reused.
pub fn build_icon_map() -> HashMap<&'static str, IconEntry> {
    let mut m = HashMap::with_capacity(256);

    // ── Common Config Files (exact filename match, lowercase) ──────────
    m.insert(".gitignore", IconEntry::new("\u{e702}", "91"));
    m.insert(".gitconfig", IconEntry::new("\u{e702}", "91"));
    m.insert(".gitattributes", IconEntry::new("\u{e702}", "91"));
    m.insert(".gitmodules", IconEntry::new("\u{e702}", "91"));
    m.insert(".gitlab-ci.yml", IconEntry::new("\u{e702}", "91"));
    m.insert(".github", IconEntry::new("\u{e702}", "90"));
    m.insert("dockerfile", IconEntry::new("\u{f308}", "94"));
    m.insert(".dockerignore", IconEntry::new("\u{f308}", "94"));
    m.insert("docker-compose.yml", IconEntry::new("\u{f308}", "94"));
    m.insert("docker-compose.yaml", IconEntry::new("\u{f308}", "94"));
    m.insert("license", IconEntry::new("\u{f718}", "90"));
    m.insert("license.md", IconEntry::new("\u{f718}", "90"));
    m.insert("license.txt", IconEntry::new("\u{f718}", "90"));
    m.insert("readme.md", IconEntry::new("\u{f48a}", "96"));
    m.insert("readme", IconEntry::new("\u{f48a}", "96"));
    m.insert("readme.txt", IconEntry::new("\u{f48a}", "96"));
    m.insert("makefile", IconEntry::new("\u{f085}", "90"));
    m.insert("cmakelists.txt", IconEntry::new("\u{f085}", "90"));
    m.insert(".editorconfig", IconEntry::new("\u{f085}", "90"));
    m.insert(".eslintrc", IconEntry::new("\u{f085}", "95"));
    m.insert(".eslintrc.js", IconEntry::new("\u{f085}", "95"));
    m.insert(".eslintrc.json", IconEntry::new("\u{f085}", "95"));
    m.insert(".eslintrc.yml", IconEntry::new("\u{f085}", "95"));
    m.insert(".prettierrc", IconEntry::new("\u{f085}", "95"));
    m.insert(".prettierrc.js", IconEntry::new("\u{f085}", "95"));
    m.insert(".prettierrc.json", IconEntry::new("\u{f085}", "95"));
    m.insert(".prettierrc.yml", IconEntry::new("\u{f085}", "95"));
    m.insert(".npmrc", IconEntry::new("\u{e71e}", "91"));
    m.insert(".nvmrc", IconEntry::new("\u{e718}", "92"));
    m.insert(".babelrc", IconEntry::new("\u{f085}", "93"));
    m.insert(".browserslistrc", IconEntry::new("\u{f085}", "90"));
    m.insert("tsconfig.json", IconEntry::new("\u{e628}", "94"));
    m.insert("jsconfig.json", IconEntry::new("\u{e781}", "93"));
    m.insert(".env", IconEntry::new("\u{f462}", "90"));
    m.insert(".env.local", IconEntry::new("\u{f462}", "90"));
    m.insert(".env.development", IconEntry::new("\u{f462}", "90"));
    m.insert(".env.production", IconEntry::new("\u{f462}", "90"));
    m.insert(".env.test", IconEntry::new("\u{f462}", "90"));
    m.insert("vagrantfile", IconEntry::new("\u{f085}", "94"));
    m.insert("gruntfile.js", IconEntry::new("\u{e74c}", "93"));
    m.insert("gulpfile.js", IconEntry::new("\u{e610}", "91"));
    m.insert("webpack.config.js", IconEntry::new("\u{f72b}", "94"));
    m.insert("rollup.config.js", IconEntry::new("\u{f085}", "91"));
    m.insert("vite.config.js", IconEntry::new("\u{f085}", "95"));
    m.insert("vite.config.ts", IconEntry::new("\u{f085}", "95"));
    m.insert("next.config.js", IconEntry::new("\u{f085}", "37"));
    m.insert("nuxt.config.js", IconEntry::new("\u{f085}", "92"));
    m.insert("angular.json", IconEntry::new("\u{e753}", "91"));
    m.insert("tailwind.config.js", IconEntry::new("\u{f085}", "36"));
    m.insert("tailwind.config.ts", IconEntry::new("\u{f085}", "36"));
    m.insert("postcss.config.js", IconEntry::new("\u{f085}", "91"));
    m.insert("jest.config.js", IconEntry::new("\u{f085}", "91"));
    m.insert("jest.config.ts", IconEntry::new("\u{f085}", "91"));
    m.insert("vitest.config.ts", IconEntry::new("\u{f085}", "92"));
    m.insert("cargo.toml", IconEntry::new("\u{e7a8}", "91"));
    m.insert("cargo.lock", IconEntry::new("\u{f023}", "90"));

    // ── Package Managers & Dependencies ────────────────────────────────
    m.insert("package.json", IconEntry::new("\u{e71e}", "91"));
    m.insert("package-lock.json", IconEntry::new("\u{f023}", "90"));
    m.insert("yarn.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("pnpm-lock.yaml", IconEntry::new("\u{f023}", "90"));
    m.insert("composer.json", IconEntry::new("\u{e608}", "90"));
    m.insert("composer.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("gemfile", IconEntry::new("\u{e21e}", "91"));
    m.insert("gemfile.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("requirements.txt", IconEntry::new("\u{e606}", "92"));
    m.insert("pipfile", IconEntry::new("\u{e606}", "92"));
    m.insert("pipfile.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("pyproject.toml", IconEntry::new("\u{e606}", "92"));
    m.insert("setup.py", IconEntry::new("\u{e606}", "92"));
    m.insert("setup.cfg", IconEntry::new("\u{e606}", "92"));
    m.insert("go.mod", IconEntry::new("\u{e626}", "96"));
    m.insert("go.sum", IconEntry::new("\u{f023}", "90"));
    m.insert("pubspec.yaml", IconEntry::new("\u{e798}", "94"));
    m.insert("pubspec.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("build.gradle", IconEntry::new("\u{e660}", "94"));
    m.insert("build.gradle.kts", IconEntry::new("\u{e660}", "94"));
    m.insert("settings.gradle", IconEntry::new("\u{e660}", "94"));
    m.insert("pom.xml", IconEntry::new("\u{e674}", "91"));
    m.insert("mix.exs", IconEntry::new("\u{e62d}", "95"));
    m.insert("mix.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("flake.nix", IconEntry::new("\u{f313}", "94"));
    m.insert("flake.lock", IconEntry::new("\u{f023}", "90"));
    m.insert("default.nix", IconEntry::new("\u{f313}", "94"));
    m.insert("shell.nix", IconEntry::new("\u{f313}", "94"));

    // ── CI/CD ──────────────────────────────────────────────────────────
    m.insert(".travis.yml", IconEntry::new("\u{e77e}", "91"));
    m.insert("jenkinsfile", IconEntry::new("\u{e767}", "91"));
    m.insert(".circleci", IconEntry::new("\u{f085}", "92"));

    // ── Programming Languages (by extension) ───────────────────────────
    // Python
    m.insert(".py", IconEntry::new("\u{e606}", "92"));
    m.insert(".pyc", IconEntry::new("\u{e606}", "90"));
    m.insert(".pyo", IconEntry::new("\u{e606}", "90"));
    m.insert(".pyd", IconEntry::new("\u{e606}", "90"));
    m.insert(".pyw", IconEntry::new("\u{e606}", "92"));
    m.insert(".pyx", IconEntry::new("\u{e606}", "92"));
    m.insert(".pxd", IconEntry::new("\u{e606}", "92"));
    m.insert(".ipynb", IconEntry::new("\u{e606}", "93"));

    // JavaScript / TypeScript
    m.insert(".js", IconEntry::new("\u{e781}", "93"));
    m.insert(".jsx", IconEntry::new("\u{e7ba}", "96"));
    m.insert(".ts", IconEntry::new("\u{e628}", "94"));
    m.insert(".tsx", IconEntry::new("\u{e7ba}", "96"));
    m.insert(".mjs", IconEntry::new("\u{e781}", "93"));
    m.insert(".cjs", IconEntry::new("\u{e781}", "93"));

    // Java / JVM
    m.insert(".java", IconEntry::new("\u{e738}", "93"));
    m.insert(".class", IconEntry::new("\u{e738}", "90"));
    m.insert(".jar", IconEntry::new("\u{e738}", "93"));
    m.insert(".war", IconEntry::new("\u{e738}", "93"));
    m.insert(".gradle", IconEntry::new("\u{e660}", "94"));

    // C / C++
    m.insert(".c", IconEntry::new("\u{e61e}", "96"));
    m.insert(".cpp", IconEntry::new("\u{e61d}", "96"));
    m.insert(".cc", IconEntry::new("\u{e61d}", "96"));
    m.insert(".cxx", IconEntry::new("\u{e61d}", "96"));
    m.insert(".c++", IconEntry::new("\u{e61d}", "96"));
    m.insert(".h", IconEntry::new("\u{e61e}", "96"));
    m.insert(".hpp", IconEntry::new("\u{e61d}", "96"));
    m.insert(".hh", IconEntry::new("\u{e61d}", "96"));
    m.insert(".hxx", IconEntry::new("\u{e61d}", "96"));
    m.insert(".h++", IconEntry::new("\u{e61d}", "96"));

    // Rust
    m.insert(".rs", IconEntry::new("\u{e7a8}", "91"));
    m.insert(".rlib", IconEntry::new("\u{e7a8}", "90"));

    // Go
    m.insert(".go", IconEntry::new("\u{e626}", "96"));

    // PHP
    m.insert(".php", IconEntry::new("\u{e608}", "95"));
    m.insert(".phtml", IconEntry::new("\u{e608}", "95"));

    // Ruby
    m.insert(".rb", IconEntry::new("\u{e21e}", "91"));
    m.insert(".erb", IconEntry::new("\u{e21e}", "91"));
    m.insert(".rake", IconEntry::new("\u{e21e}", "91"));
    m.insert(".gemspec", IconEntry::new("\u{e21e}", "91"));

    // Swift
    m.insert(".swift", IconEntry::new("\u{e755}", "93"));

    // Kotlin
    m.insert(".kt", IconEntry::new("\u{e634}", "93"));
    m.insert(".kts", IconEntry::new("\u{e634}", "93"));

    // Lua
    m.insert(".lua", IconEntry::new("\u{e620}", "94"));

    // Dart
    m.insert(".dart", IconEntry::new("\u{e798}", "94"));

    // Haskell
    m.insert(".hs", IconEntry::new("\u{e61f}", "95"));
    m.insert(".lhs", IconEntry::new("\u{e61f}", "95"));
    m.insert(".cabal", IconEntry::new("\u{e61f}", "95"));

    // R
    m.insert(".r", IconEntry::new("\u{f25d}", "94"));
    m.insert(".rmd", IconEntry::new("\u{f25d}", "94"));

    // Julia
    m.insert(".jl", IconEntry::new("\u{e624}", "95"));

    // Perl
    m.insert(".pl", IconEntry::new("\u{e769}", "94"));
    m.insert(".pm", IconEntry::new("\u{e769}", "94"));

    // Shell
    m.insert(".sh", IconEntry::new("\u{f489}", "32"));
    m.insert(".bash", IconEntry::new("\u{f489}", "32"));
    m.insert(".zsh", IconEntry::new("\u{f489}", "32"));
    m.insert(".fish", IconEntry::new("\u{f489}", "32"));
    m.insert(".ksh", IconEntry::new("\u{f489}", "32"));
    m.insert(".csh", IconEntry::new("\u{f489}", "32"));
    m.insert(".tcsh", IconEntry::new("\u{f489}", "32"));
    m.insert(".ps1", IconEntry::new("\u{f489}", "94"));
    m.insert(".psm1", IconEntry::new("\u{f489}", "94"));
    m.insert(".psd1", IconEntry::new("\u{f489}", "94"));
    m.insert(".bat", IconEntry::new("\u{f489}", "92"));
    m.insert(".cmd", IconEntry::new("\u{f489}", "92"));

    // C# / .NET
    m.insert(".cs", IconEntry::new("\u{f81a}", "94"));
    m.insert(".csx", IconEntry::new("\u{f81a}", "94"));
    m.insert(".csproj", IconEntry::new("\u{f81a}", "94"));
    m.insert(".sln", IconEntry::new("\u{f81a}", "95"));

    // F#
    m.insert(".fs", IconEntry::new("\u{e7a7}", "94"));
    m.insert(".fsx", IconEntry::new("\u{e7a7}", "94"));
    m.insert(".fsi", IconEntry::new("\u{e7a7}", "94"));
    m.insert(".fsproj", IconEntry::new("\u{e7a7}", "94"));

    // VB
    m.insert(".vb", IconEntry::new("\u{f81a}", "94"));
    m.insert(".vbs", IconEntry::new("\u{f81a}", "94"));

    // Scala
    m.insert(".scala", IconEntry::new("\u{e737}", "91"));
    m.insert(".sc", IconEntry::new("\u{e737}", "91"));
    m.insert(".sbt", IconEntry::new("\u{e737}", "91"));

    // Clojure
    m.insert(".clj", IconEntry::new("\u{e768}", "92"));
    m.insert(".cljs", IconEntry::new("\u{e768}", "92"));
    m.insert(".cljc", IconEntry::new("\u{e768}", "92"));
    m.insert(".edn", IconEntry::new("\u{e768}", "92"));

    // Erlang
    m.insert(".erl", IconEntry::new("\u{e7b1}", "95"));
    m.insert(".hrl", IconEntry::new("\u{e7b1}", "95"));

    // Elixir
    m.insert(".ex", IconEntry::new("\u{e62d}", "95"));
    m.insert(".exs", IconEntry::new("\u{e62d}", "95"));
    m.insert(".eex", IconEntry::new("\u{e62d}", "95"));
    m.insert(".heex", IconEntry::new("\u{e62d}", "95"));
    m.insert(".leex", IconEntry::new("\u{e62d}", "95"));

    // Vim
    m.insert(".vim", IconEntry::new("\u{e62b}", "92"));
    m.insert(".vimrc", IconEntry::new("\u{e62b}", "92"));

    // Zig
    m.insert(".zig", IconEntry::new("\u{f0e7}", "93"));

    // Nim
    m.insert(".nim", IconEntry::new("\u{f0e7}", "93"));
    m.insert(".nimble", IconEntry::new("\u{f0e7}", "93"));

    // Crystal
    m.insert(".cr", IconEntry::new("\u{e62f}", "37"));

    // OCaml
    m.insert(".ml", IconEntry::new("\u{03bb}", "93")); // λ
    m.insert(".mli", IconEntry::new("\u{03bb}", "93"));

    // Verilog / VHDL
    m.insert(".v", IconEntry::new("\u{f085}", "94"));
    m.insert(".sv", IconEntry::new("\u{f085}", "94"));
    m.insert(".vhd", IconEntry::new("\u{f085}", "94"));
    m.insert(".vhdl", IconEntry::new("\u{f085}", "94"));

    // Assembly
    m.insert(".asm", IconEntry::new("\u{f471}", "91"));
    m.insert(".s", IconEntry::new("\u{f471}", "91"));
    m.insert(".S", IconEntry::new("\u{f471}", "91"));

    // Nix
    m.insert(".nix", IconEntry::new("\u{f313}", "94"));

    // Terraform
    m.insert(".tf", IconEntry::new("\u{f085}", "95"));
    m.insert(".tfvars", IconEntry::new("\u{f085}", "95"));

    // ── Web & Frontend ─────────────────────────────────────────────────
    m.insert(".html", IconEntry::new("\u{e736}", "95"));
    m.insert(".htm", IconEntry::new("\u{e736}", "95"));
    m.insert(".xhtml", IconEntry::new("\u{e736}", "95"));
    m.insert(".css", IconEntry::new("\u{e749}", "36"));
    m.insert(".scss", IconEntry::new("\u{e749}", "36"));
    m.insert(".sass", IconEntry::new("\u{e749}", "36"));
    m.insert(".less", IconEntry::new("\u{e749}", "36"));
    m.insert(".styl", IconEntry::new("\u{e749}", "36"));
    m.insert(".vue", IconEntry::new("\u{e6a0}", "92"));
    m.insert(".svelte", IconEntry::new("\u{e697}", "91"));
    m.insert(".astro", IconEntry::new("\u{e697}", "91"));
    m.insert(".wasm", IconEntry::new("\u{e6a1}", "95"));

    // ── Data & Config ──────────────────────────────────────────────────
    m.insert(".json", IconEntry::new("\u{e60b}", "94"));
    m.insert(".jsonc", IconEntry::new("\u{e60b}", "94"));
    m.insert(".json5", IconEntry::new("\u{e60b}", "94"));
    m.insert(".jsonl", IconEntry::new("\u{e60b}", "94"));
    m.insert(".ndjson", IconEntry::new("\u{e60b}", "94"));
    m.insert(".xml", IconEntry::new("\u{e619}", "91"));
    m.insert(".xsl", IconEntry::new("\u{e619}", "91"));
    m.insert(".xsd", IconEntry::new("\u{e619}", "91"));
    m.insert(".yml", IconEntry::new("\u{e60b}", "94"));
    m.insert(".yaml", IconEntry::new("\u{e60b}", "94"));
    m.insert(".toml", IconEntry::new("\u{e60b}", "94"));
    m.insert(".ini", IconEntry::new("\u{f085}", "90"));
    m.insert(".env", IconEntry::new("\u{f462}", "90"));
    m.insert(".conf", IconEntry::new("\u{f085}", "90"));
    m.insert(".cfg", IconEntry::new("\u{f085}", "90"));
    m.insert(".properties", IconEntry::new("\u{f085}", "90"));
    m.insert(".sql", IconEntry::new("\u{f1c0}", "93"));
    m.insert(".db", IconEntry::new("\u{f1c0}", "93"));
    m.insert(".sqlite", IconEntry::new("\u{f1c0}", "93"));
    m.insert(".sqlite3", IconEntry::new("\u{f1c0}", "93"));
    m.insert(".graphql", IconEntry::new("\u{e662}", "95"));
    m.insert(".gql", IconEntry::new("\u{e662}", "95"));
    m.insert(".proto", IconEntry::new("\u{f085}", "94"));
    m.insert(".csv", IconEntry::new("\u{f1c3}", "92"));
    m.insert(".tsv", IconEntry::new("\u{f1c3}", "92"));
    m.insert(".hcl", IconEntry::new("\u{f085}", "95"));

    // ── Documents ──────────────────────────────────────────────────────
    m.insert(".md", IconEntry::new("\u{e73e}", "96"));
    m.insert(".markdown", IconEntry::new("\u{e73e}", "96"));
    m.insert(".mdx", IconEntry::new("\u{e73e}", "96"));
    m.insert(".txt", IconEntry::new("\u{f15c}", "37"));
    m.insert(".pdf", IconEntry::new("\u{f1c1}", "31"));
    m.insert(".doc", IconEntry::new("\u{f1c2}", "94"));
    m.insert(".docx", IconEntry::new("\u{f1c2}", "94"));
    m.insert(".xls", IconEntry::new("\u{f1c3}", "92"));
    m.insert(".xlsx", IconEntry::new("\u{f1c3}", "92"));
    m.insert(".ppt", IconEntry::new("\u{f1c4}", "91"));
    m.insert(".pptx", IconEntry::new("\u{f1c4}", "91"));
    m.insert(".log", IconEntry::new("\u{f15c}", "90"));
    m.insert(".tex", IconEntry::new("\u{f15c}", "92"));
    m.insert(".bib", IconEntry::new("\u{f15c}", "93"));
    m.insert(".rtf", IconEntry::new("\u{f1c2}", "94"));
    m.insert(".odt", IconEntry::new("\u{f1c2}", "94"));
    m.insert(".ods", IconEntry::new("\u{f1c3}", "92"));
    m.insert(".odp", IconEntry::new("\u{f1c4}", "91"));
    m.insert(".epub", IconEntry::new("\u{f02d}", "91"));
    m.insert(".rst", IconEntry::new("\u{e73e}", "96"));
    m.insert(".org", IconEntry::new("\u{e633}", "92"));
    m.insert(".adoc", IconEntry::new("\u{e73e}", "96"));
    m.insert(".man", IconEntry::new("\u{f15c}", "90"));

    // ── Media - Images ─────────────────────────────────────────────────
    m.insert(".png", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".jpg", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".jpeg", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".svg", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".gif", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".webp", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".bmp", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".ico", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".tiff", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".tif", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".psd", IconEntry::new("\u{e7b8}", "94"));
    m.insert(".ai", IconEntry::new("\u{e7b4}", "93"));
    m.insert(".eps", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".raw", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".cr2", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".nef", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".heic", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".heif", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".avif", IconEntry::new("\u{f1c5}", "93"));
    m.insert(".jxl", IconEntry::new("\u{f1c5}", "93"));

    // ── Media - Audio ──────────────────────────────────────────────────
    m.insert(".mp3", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".wav", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".flac", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".aac", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".ogg", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".m4a", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".wma", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".opus", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".mid", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".midi", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".aiff", IconEntry::new("\u{f1c7}", "95"));
    m.insert(".ape", IconEntry::new("\u{f1c7}", "95"));

    // ── Media - Video ──────────────────────────────────────────────────
    m.insert(".mp4", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".mov", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".avi", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".mkv", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".webm", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".flv", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".wmv", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".m4v", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".mpg", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".mpeg", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".3gp", IconEntry::new("\u{f1c8}", "95"));
    m.insert(".ts", IconEntry::new("\u{f1c8}", "95")); // NOTE: conflicts with TypeScript, extension wins last

    // ── Compressed & Archives ──────────────────────────────────────────
    m.insert(".zip", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".tar", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".gz", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".bz2", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".xz", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".zst", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".lz", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".lz4", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".lzma", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".7z", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".rar", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".tgz", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".tbz2", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".txz", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".iso", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".dmg", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".img", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".cab", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".cpio", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".ar", IconEntry::new("\u{f1c6}", "95"));
    m.insert(".deb", IconEntry::new("\u{f187}", "91"));
    m.insert(".rpm", IconEntry::new("\u{f187}", "91"));
    m.insert(".apk", IconEntry::new("\u{e70e}", "92"));
    m.insert(".snap", IconEntry::new("\u{f187}", "92"));
    m.insert(".flatpak", IconEntry::new("\u{f187}", "94"));
    m.insert(".appimage", IconEntry::new("\u{f187}", "92"));

    // ── Executables & Binaries ─────────────────────────────────────────
    m.insert(".exe", IconEntry::new("\u{f17a}", "92"));
    m.insert(".app", IconEntry::new("\u{f179}", "92"));
    m.insert(".dll", IconEntry::new("\u{f17a}", "90"));
    m.insert(".so", IconEntry::new("\u{f17c}", "90"));
    m.insert(".dylib", IconEntry::new("\u{f179}", "90"));
    m.insert(".bin", IconEntry::new("\u{f489}", "90"));
    m.insert(".msi", IconEntry::new("\u{f17a}", "92"));
    m.insert(".out", IconEntry::new("\u{f489}", "90"));
    m.insert(".elf", IconEntry::new("\u{f489}", "90"));
    m.insert(".o", IconEntry::new("\u{f471}", "90"));
    m.insert(".a", IconEntry::new("\u{f471}", "90"));

    // ── Fonts ──────────────────────────────────────────────────────────
    m.insert(".ttf", IconEntry::new("\u{f031}", "37"));
    m.insert(".otf", IconEntry::new("\u{f031}", "37"));
    m.insert(".woff", IconEntry::new("\u{f031}", "37"));
    m.insert(".woff2", IconEntry::new("\u{f031}", "37"));
    m.insert(".eot", IconEntry::new("\u{f031}", "37"));

    // ── 3D & CAD ───────────────────────────────────────────────────────
    m.insert(".obj", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".fbx", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".stl", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".blend", IconEntry::new("\u{f1b2}", "93"));
    m.insert(".3ds", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".dae", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".gltf", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".glb", IconEntry::new("\u{f1b2}", "95"));
    m.insert(".usdz", IconEntry::new("\u{f1b2}", "95"));

    // ── Security / Crypto ──────────────────────────────────────────────
    m.insert(".key", IconEntry::new("\u{f084}", "91"));
    m.insert(".pem", IconEntry::new("\u{f084}", "91"));
    m.insert(".crt", IconEntry::new("\u{f084}", "91"));
    m.insert(".cert", IconEntry::new("\u{f084}", "91"));
    m.insert(".cer", IconEntry::new("\u{f084}", "91"));
    m.insert(".ca", IconEntry::new("\u{f084}", "91"));
    m.insert(".pub", IconEntry::new("\u{f084}", "91"));
    m.insert(".gpg", IconEntry::new("\u{f084}", "91"));
    m.insert(".pgp", IconEntry::new("\u{f084}", "91"));
    m.insert(".asc", IconEntry::new("\u{f084}", "91"));
    m.insert(".sig", IconEntry::new("\u{f084}", "91"));
    m.insert(".p12", IconEntry::new("\u{f084}", "91"));
    m.insert(".pfx", IconEntry::new("\u{f084}", "91"));
    m.insert(".csr", IconEntry::new("\u{f084}", "91"));

    // ── Other / Misc ───────────────────────────────────────────────────
    m.insert(".lock", IconEntry::new("\u{f023}", "90"));
    m.insert(".bak", IconEntry::new("\u{f0e2}", "90"));
    m.insert(".tmp", IconEntry::new("\u{f0e2}", "90"));
    m.insert(".temp", IconEntry::new("\u{f0e2}", "90"));
    m.insert(".swp", IconEntry::new("\u{f0e2}", "90"));
    m.insert(".swo", IconEntry::new("\u{f0e2}", "90"));
    m.insert(".cache", IconEntry::new("\u{f0e2}", "90"));
    m.insert(".pid", IconEntry::new("\u{f085}", "90"));
    m.insert(".sock", IconEntry::new("\u{f6a7}", "35"));
    m.insert(".patch", IconEntry::new("\u{f440}", "92"));
    m.insert(".diff", IconEntry::new("\u{f440}", "92"));

    // TypeScript should take priority over video .ts
    // Re-insert TypeScript to override the video .ts entry
    m.insert(".ts", IconEntry::new("\u{e628}", "94"));

    m
}

/// Look up the icon for a file by name, extension, and file type.
/// `name` is the file name (just the filename, not the full path).
/// `is_dir` indicates whether the entry is a directory.
/// `is_hidden` indicates whether the entry starts with '.'.
/// `is_symlink` indicates whether the entry is a symbolic link.
/// `is_executable` indicates whether the entry has execute permission.
pub fn get_icon<'a>(
    icon_map: &'a HashMap<&str, IconEntry>,
    name: &str,
    is_dir: bool,
    is_hidden: bool,
    is_symlink: bool,
    is_executable: bool,
    is_pipe: bool,
    is_socket: bool,
    is_block_device: bool,
    is_char_device: bool,
) -> &'a IconEntry {
    // Special file types first
    if is_pipe {
        return &PIPE_ICON;
    }
    if is_socket {
        return &SOCKET_ICON;
    }
    if is_block_device {
        return &BLOCK_DEV_ICON;
    }
    if is_char_device {
        return &CHAR_DEV_ICON;
    }

    // Directories
    if is_dir {
        // Check for special directory names
        let lower = name.to_lowercase();
        if let Some(entry) = icon_map.get(lower.as_str()) {
            return entry;
        }
        if is_hidden {
            return &DIR_HIDDEN_ICON;
        }
        return &DIR_ICON;
    }

    // Symlinks (show the symlink icon if it can't be resolved to a type)
    if is_symlink {
        // Still try to match by name/extension, fall back to symlink icon
        let lower = name.to_lowercase();
        if let Some(entry) = icon_map.get(lower.as_str()) {
            return entry;
        }
        // Try compound extensions (e.g., .tar.gz)
        if let Some(idx) = lower.find('.') {
            if let Some(idx2) = lower[idx + 1..].find('.') {
                let compound = &lower[idx + 1 + idx2..];
                if let Some(entry) = icon_map.get(compound) {
                    return entry;
                }
            }
        }
        if let Some(dot_pos) = lower.rfind('.') {
            let ext = &lower[dot_pos..];
            if let Some(entry) = icon_map.get(ext) {
                return entry;
            }
        }
        return &SYMLINK_ICON;
    }

    // Regular files: check exact filename, then compound extension, then extension
    let lower = name.to_lowercase();

    // Exact filename match
    if let Some(entry) = icon_map.get(lower.as_str()) {
        return entry;
    }

    // Compound extension (e.g., .tar.gz, .spec.ts)
    if let Some(idx) = lower.find('.') {
        if let Some(idx2) = lower[idx + 1..].find('.') {
            let compound_ext = &lower[idx + 1 + idx2..];
            if let Some(entry) = icon_map.get(compound_ext) {
                return entry;
            }
        }
    }

    // Simple extension
    if let Some(dot_pos) = lower.rfind('.') {
        let ext = &lower[dot_pos..];
        if let Some(entry) = icon_map.get(ext) {
            return entry;
        }
    }

    // Executable files
    if is_executable {
        return &EXEC_ICON;
    }

    &DEFAULT_ICON
}
