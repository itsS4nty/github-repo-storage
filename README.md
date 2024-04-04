# Overview
The purpose of this tool to streamline the integration of GitHub repository information into personal or organizational websites. By leveraging the GitHub API, this application automates the retrieval of repository details such as programming languages used, commit counts, and other relevant metadata, directly inserting this data into a specified database. This process not only facilitates the dynamic display of up-to-date repository information on web pages but also significantly reduces the overhead associated with manual data entry and frequent API calls.

## Features
- Retrieve repository data from GitHub.
- Filter and include repositories containing a designated branch (detailed explanation available [here](#how-it-works)).
- Identify and catalog the programming languages utilized within each repository.
- Quantify the total number of commits for each repository.
- Aggregate projects that are interconnected or related.

## Requirements
- Rust: make sure you have Rust and Cargo installed on your machine.
- GitHub Personal Access Token (PAT): required to be able to get your repositories.

## Installation
1. Clone the repository:

```bash
git clone https://github.com/itsS4nty/github-repo-storage.git
cd github-repo-storage
```

2. Create your GitHub Personal Access Token (PAT):
- Generate a new token by following [these instructions](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens).
- Store this token securely; it will be necessary to include it in the `.env` file.

## Configure the environment variables
Create an `.env` file in the project's root directory and follow the structure of the [.env.example](.env.example).

## How it works
This tool is designed to selectively synchronize your GitHub repositories with a database, based on specific criteria defined in your environment settings. Through the use of a .env configuration, you can specify a branch name (e.g., `FEATURE_BRANCH`). Only repositories containing this branch will be integrated into the database. This functionality allows for precise control over which projects are included, accommodating scenarios where you might not wish to synchronize all your GitHub projects.

If your goal is to include all your projects, you have two options:

- Fork this tool and modify the code to suit your preferences.
- Set the branch name in the .env file to main, which typically represents the default branch in many projects.

The project naming convention is designed to transform repository names into more readable titles. For instance, a repository named `my-calculator_frontend` will be converted to "My Calculator". This transformation is handled automatically by scripts within the tool, streamlining the process for ease of use. Should you require a different naming scheme, the scripts can be adjusted to meet your specific needs.

Additionally, the tool intelligently groups related projects under a single object if they share a common base name (e.g., `my-calculator_frontend` and `my-calculator_backend`). Within this object, each project is accessible via the components property, allowing for a consolidated view of related repositories.

This approach not only organizes your projects more effectively but also enhances the accessibility and management of your project data within the database.

### Examples:

#### my-calculator:
```rust
{
    name: "My Calculator",
    components: {
        general: {
            name: "my-calculator",
            languages: ["typescript", "html", "css"],
            html_url: "repo-url",
            description: "repo-description",
            homepage: "deployed-page",
            commits: number-of-commits,
            updated_at: "last-modified",
            _type: "General"
        }
    }
}
```
#### my-calculator_frontend & my-calculator_backend:
```rust
{
    name: "My Calculator",
    components: {
        frontend: {
            name: "my-calculator_frontend",
            languages: ["typescript", "html", "css"],
            html_url: "repo-url",
            description: "repo-description",
            homepage: "deployed-page",
            commits: number-of-commits,
            updated_at: "last-modified",
            _type: "frontend"
        },
        backend: {
            name: "my-calculator_backend",
            languages: ["rust"],
            html_url: "repo-url",
            description: "repo-description",
            homepage: "deployed-page",
            commits: number-of-commits,
            updated_at: "last-modified",
            _type: "backend"
        }
    }
}
```

## Usage
To initiate and run the project, follow the steps below:
```bash
cargo build
cargo run
```
Additionally, consider scheduling a cron job to automate the execution of this project at regular intervals.



## Project structure

The project is organized into several modules, each serving a specific purpose within the application's architecture:

- **client/**: This module configures the HTTP client used to make API requests.

- **database/**: This module contains all logic related to database interactions, including queries and connections, essential for the application's data persistence.

- **github/**: Contains the logic for direct interactions with the GitHub API, including fetching repository data and metadata.

- **models/**: Defines the data structures used across the application. It's further divided into:
  - **combined_repository_info.rs**: For the `CombinedRepositoryInfo` model.
  - **repository_info.rs**: For the `RepositoryInfo` model.

- **utils/**: Houses utility functions that provide common functionalities like string manipulation and data formatting.

- **main.rs**: The entry point to the application, orchestrating the flow of data and control across the modules.