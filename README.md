# Clean Axum Demo 🚀

![Clean Architecture](https://img.shields.io/badge/Clean%20Architecture-Rust%20Axum-blue)

Welcome to the **Clean Axum Demo** repository! This project demonstrates a clean architecture approach for building a Rust-based API server using the Axum framework. This README will guide you through the setup, usage, and structure of the project. 

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Releases](#releases)

## Introduction

The Clean Axum Demo showcases how to create a well-structured API server in Rust using Axum. Clean architecture emphasizes separation of concerns, making your code easier to manage and test. This project serves as a starting point for developers looking to build robust and maintainable applications.

## Features

- **Modular Design**: Organized into distinct layers for better maintainability.
- **Easy to Extend**: Add new features without affecting existing functionality.
- **Testable**: Each component can be tested independently.
- **Performance**: Leverages Rust's speed and safety.

## Getting Started

To get started with the Clean Axum Demo, follow these steps:

### Prerequisites

- Install Rust. You can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
- Ensure you have Cargo, Rust's package manager, installed.

### Clone the Repository

Open your terminal and run:

```bash
git clone https://github.com/Unico88/clean_axum_demo.git
cd clean_axum_demo
```

### Install Dependencies

Run the following command to install the required dependencies:

```bash
cargo build
```

### Run the Server

To start the API server, execute:

```bash
cargo run
```

The server will run on `http://localhost:3000` by default.

## Project Structure

The project is organized into several key directories:

- **src/**: Contains the main application code.
  - **api/**: Handles API routes and request handling.
  - **domain/**: Contains business logic and domain models.
  - **infrastructure/**: Manages external dependencies like databases and services.
  - **presentation/**: Manages the user interface and API responses.
- **tests/**: Contains integration tests to ensure the application works as expected.

## Usage

Once the server is running, you can interact with it using tools like Postman or cURL. Here are some example endpoints:

### Get All Items

```http
GET /items
```

### Create a New Item

```http
POST /items
Content-Type: application/json

{
  "name": "New Item",
  "description": "Description of the new item"
}
```

### Get Item by ID

```http
GET /items/{id}
```

### Update an Item

```http
PUT /items/{id}
Content-Type: application/json

{
  "name": "Updated Item",
  "description": "Updated description"
}
```

### Delete an Item

```http
DELETE /items/{id}
```

## Contributing

We welcome contributions to the Clean Axum Demo project! If you want to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them.
4. Push your branch to your forked repository.
5. Create a pull request to the main repository.

Please ensure your code adheres to the project's coding standards and includes tests where applicable.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Releases

For the latest releases, visit [this link](https://github.com/Unico88/clean_axum_demo/releases). You can download the files and execute them as needed.

To keep up with updates, you can also check the "Releases" section in the GitHub repository.

## Conclusion

Thank you for checking out the Clean Axum Demo! We hope this project helps you understand clean architecture principles in Rust and Axum. If you have any questions or feedback, feel free to reach out or open an issue in the repository.

Happy coding! 🎉