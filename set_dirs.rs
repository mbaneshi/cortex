use std::fs;

fn main() {
    let dirs = vec![
        "src",
        "src/cli",
        "src/web",
        "src/db",
        "src/services",
        "src/utils",
        "tests",
    ];

    let files = vec![
        "src/main.rs",
        "src/cli/mod.rs",
        "src/cli/command.rs",
        "src/cli/parser.rs",
        "src/web/mod.rs",
        "src/web/routes.rs",
        "src/web/handlers.rs",
        "src/db/mod.rs",
        "src/db/models.rs",
        "src/db/repository.rs",
        "src/services/mod.rs",
        "src/services/search_service.rs",
        "src/services/file_service.rs",
        "src/utils/mod.rs",
        "src/utils/logging.rs",
        "src/config.rs",
        "tests/integration_tests.rs",
        "Cargo.toml",
        "README.md",
    ];

    for dir in dirs {
        fs::create_dir_all(dir).unwrap_or_else(|err| {
            eprintln!("Failed to create directory {}: {}", dir, err);
        });
    }

    for file in files {
        fs::write(file, "").unwrap_or_else(|err| {
            eprintln!("Failed to create file {}: {}", file, err);
        });
    }

    println!("Directory structure created successfully!");
}
