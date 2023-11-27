# AOC2023 - Advent of Code 2023

## Overview

Welcome to AOC 2023, a project that combines the power Fermyon Spin with Rust, JavaScript, Zig, TypeScript, HTML, and ReactJS to provide a comprehensive solution for Advent of Code challenges. This repository is structured as a Next.js UI powered by ReactJS, with a Rust API serving specific endpoints for each daily challenge.

## Project Structure

- `ui/`: Next.js UI built with ReactJS.
- `api/`: Rust API providing endpoints for each daily challenge.

## Technologies Used

- **Rust**: High-performance programming language for building the API.
- **JavaScript/TypeScript**: Used in the UI for dynamic and responsive user interfaces.
- **Zig**: A system programming language used in specific components.
- **HTML**: Standard markup language for the project's web pages.
- **ReactJS**: A JavaScript library for building user interfaces, utilized in the Next.js UI.

## Getting Started

Follow these steps to set up the project locally:

1. **Clone the repository:**

    ```bash
    git clone https://github.com/your-username/AOC2023.git
    cd AOC2023
    cd ui && npm install && npm build && cd ..
    spin build --up
    ```

2. **Install dependencies:**

   For the UI:

    ```bash
    cd ui
    npm install
    ```

   For the API:

    ```bash
    cd api
    cargo build
    ```

3. **Run the project:**

   For the UI:

    ```bash
    cd ui
    npm run dev
    ```

   For the API:

    ```bash
    cd api
    cargo run
    ```

   The UI should be accessible at `http://localhost:3000`, and the API at `http://localhost:8000`.

## API Endpoints

- **Day 1 Challenge:**
  - Endpoint: `/api/day-1`
  - Description: Provides solutions for the Day 1 challenge.

- **Day 2 Challenge:**
  - Endpoint: `/api/day-2`
  - Description: Provides solutions for the Day 2 challenge.

... *(Repeat for each day)*

## Contributions

Contributions are welcome! If you'd like to contribute, please follow the guidelines outlined in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE). See the LICENSE file for details.

---

Feel free to explore, challenge yourself, and enjoy solving the Advent of Code puzzles with AOC2023!
