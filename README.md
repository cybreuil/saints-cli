# saints-cli

**saints-cli**, has the ambition to become the go-to command-line tool for discovering the saint of the day.

## 🌟 Project Ambitions

Driven by a passion for programming and catholic culture, this project aims to provide users with a simple and efficient way to access information about saints celebrated on any given day. Whether for personal interest, educational purposes, or cultural enrichment, **saints-cli** aspires to be a reliable companion for enthusiasts of hagiography.

This project is intended to evolve: new features, improved usability, and integration with package managers (AUR, Homebrew, etc.) are all on the roadmap. Maybe a graphical interface in the future? Who knows!

This project is open to suggestions from the community to enhance its features and usability.

## 🚀 Planned Features

- Display the saint of the day with a short description.
- Option to look up the saint for a specific date.
- Use of a reliable API or database for saint information, i will propably use https://catholicreadings.org/api/saints/
- Support for multiple languages (starting with English and French).
- Clear and ergonomic command-line interface.

## 📦 Installation

*(To be completed when the project is published on crates.io or the AUR)*

For now, clone and run locally:
```sh
git clone https://github.com/cybreuil/saints-cli.git
cd saints-cli
cargo run
```

## 🛠️ Technologies

- Rust

We'll see for the rest probably :
- clap (CLI argument parsing)
- reqwest (HTTP requests)
- serde (JSON parsing)
- chrono (date handling)

## 📄 License

This project is licensed under the MIT License.  
Copyright (c) 2024 Cyrille Breuil
